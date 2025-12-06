use rand::Rng;
use crate::get_orders;
use crate::resource::{Company, Frequency};
use crate::simulated_annealing::route::OrderIndex;
use super::week::DayEnum;
pub struct OrderFlags {
    orders: Vec<u8>,
}


/// Maybe use the left 3 bits to store the frequency of each order
impl OrderFlags {
    pub fn new(size: usize) -> Self{
        OrderFlags {
            orders: vec![0; size]
        }
    }

    pub fn add_order(&mut self, order: OrderIndex, day: DayEnum){
        match day {
            DayEnum::Monday     => {self.orders[order] = self.orders[order] | 0b00000001}
            DayEnum::Tuesday    => {self.orders[order] = self.orders[order] | 0b00000010}
            DayEnum::Wednesday  => {self.orders[order] = self.orders[order] | 0b00000100}
            DayEnum::Thursday   => {self.orders[order] = self.orders[order] | 0b00001000}
            DayEnum::Friday     => {self.orders[order] = self.orders[order] | 0b00010000}
        }
    }
    pub fn remove_order(&mut self, order: OrderIndex, day: DayEnum){
        match day {
            DayEnum::Monday     => {self.orders[order] = self.orders[order] & 0b00011110}
            DayEnum::Tuesday    => {self.orders[order] = self.orders[order] & 0b00011101}
            DayEnum::Wednesday  => {self.orders[order] = self.orders[order] & 0b00011011}
            DayEnum::Thursday   => {self.orders[order] = self.orders[order] & 0b00010111}
            DayEnum::Friday     => {self.orders[order] = self.orders[order] & 0b00001111}
        }
    }

    pub fn get_random_allowed_day<R: Rng+?Sized>(&self, order_index: OrderIndex, rng: &mut R) -> Option<DayEnum> {
        let order = &get_orders()[order_index];
        let flags = self.orders[order_index];
        match order.frequency{
            Frequency::Once => {
                if self.orders[order_index] == 0{
                    Some(rng.random())
                } else {
                    None
                }
            }
            Frequency::Twice => {
                let flags = self.orders[order_index] & 0b1_1111;
                match flags {
                    0b10000 => Some(DayEnum::Thursday),
                    0b01000 => Some(DayEnum::Friday),
                    0b00010 => Some(DayEnum::Monday),
                    0b00001 => Some(DayEnum::Tuesday),
                    0b00100 => panic!("An order with frequency 2 has been put on Wednesday"),
                    0b00000 => {
                        match rng.random_range(0..4){
                            0 => Some(DayEnum::Monday),
                            1 => Some(DayEnum::Tuesday),
                            2 => Some(DayEnum::Thursday),
                            3 => Some(DayEnum::Friday),
                            _ => None
                        }
                    }
                    _ => unreachable!()
                }
            }
            Frequency::Thrice => {
                let mask = 0b10000 | 0b00100 | 0b00001;
                let available:u8 = mask & !flags;
                if available == 0{
                    return None
                }


                // find how many are available
                let mut count = 0;
                for bit in &[
                    0b10000,
                    0b00100,
                    0b00001
                ] {
                    if available & bit != 0 {
                        count += 1;
                    }
                }

                // get a random number
                let choice_idx = rng.random_range(0..count);
                let mut idx = choice_idx;
                for (bit, day) in &[
                    (0b10000, DayEnum::Monday),
                    (0b00100, DayEnum::Wednesday),
                    (0b00001, DayEnum::Friday),
                ] {
                    // iterate to the random number
                    if available & bit != 0 {
                        if idx == 0 {
                            return Some(*day);
                        }
                        idx -= 1;
                    }
                }

                unreachable!()
            }
            Frequency::FourTimes => {
                let mask = 0b10000 | 0b01000 | 0b00100 | 0b00010 | 0b00001;
                let available = mask & !flags;
                if available == 0 {
                    return None;
                }

                // count available days
                let mut count = 0;
                for bit in &[
                    0b10000,
                    0b01000,
                    0b00100,
                    0b00010,
                    0b00001,
                ] {
                    if available & bit != 0 {
                        count += 1;
                    }
                }

                // get a random day
                let choice_idx = rng.random_range(0..count);
                let mut idx = choice_idx;
                for (bit, day) in &[
                    (0b10000, DayEnum::Monday),
                    (0b01000, DayEnum::Tuesday),
                    (0b00100, DayEnum::Wednesday),
                    (0b00010, DayEnum::Thursday),
                    (0b00001, DayEnum::Friday),
                ] {
                    if available & bit != 0 {
                        if idx == 0 {
                            return Some(*day);
                        }
                        idx -= 1;
                    }
                }

                unreachable!()
            }
        }
    }
    pub fn get_filled_count(&self, order_index: OrderIndex) -> u32 {
        self.orders[order_index].count_ones()
    }
}