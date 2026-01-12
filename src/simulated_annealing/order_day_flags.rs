use super::week::DayEnum;
use crate::get_orders;
use crate::resource::{Frequency};
use crate::simulated_annealing::route::OrderIndex;
use rand::Rng;
#[derive(Clone)]
pub struct OrderFlags {
    orders: Vec<u8>,
}

/// Maybe use the left 3 bits to store the frequency of each order
impl OrderFlags {
    pub fn new(size: usize) -> Self {
        OrderFlags {
            orders: vec![0; size],
        }
    }

    pub fn add_order(&mut self, order: OrderIndex, day: DayEnum) {
        // First we get the order, and do an and operator with the day.
        // This works like a bitmask.
        // If this value is not equal to 0,
        // there was already something on the day where a new order would have been added.
        debug_assert_eq!(self.orders[order] & Self::day_to_flags(day), 0);
        match day {
            DayEnum::Monday =>   self.orders[order] |= 0b00010000,
            DayEnum::Tuesday =>  self.orders[order] |= 0b00001000,
            DayEnum::Wednesday =>self.orders[order] |= 0b00000100,
            DayEnum::Thursday => self.orders[order] |= 0b00000010,
            DayEnum::Friday =>   self.orders[order] |= 0b00000001,
        }
    }
    pub fn remove_order(&mut self, order: OrderIndex, day: DayEnum) {
        // First we get the order, and do an and operator with the day.
        // This works like a bitmask.
        // If this value is equal to 0,
        // we did not have anything on the day where the order would have been removed
        debug_assert_ne!(self.orders[order] & Self::day_to_flags(day), 0);
        match day {
            DayEnum::Monday =>   self.orders[order] &= 0b00001111,
            DayEnum::Tuesday =>  self.orders[order] &= 0b00010111,
            DayEnum::Wednesday =>self.orders[order] &= 0b00011011,
            DayEnum::Thursday => self.orders[order] &= 0b00011101,
            DayEnum::Friday =>   self.orders[order] &= 0b00011110,
        }
    }

    pub fn get_random_allowed_day<R: Rng + ?Sized>(
        &self,
        order_index: OrderIndex,
        rng: &mut R,
    ) -> Option<DayEnum> {
        let order = &get_orders()[order_index];
        let flags = self.orders[order_index] & 0b1_1111;
        OrderFlags::_get_random_allowed_day(flags, order.frequency, rng)
    }

    pub fn get_random_day_to_shift_to<R: Rng + ?Sized>(
        &self,
        order_index: OrderIndex,
        shift_from: DayEnum,
        rng: &mut R,
    ) -> Option<DayEnum> {
        let order = &get_orders()[order_index];
        let flags = (self.orders[order_index] & 0b1_1111) ^ Self::day_to_flags(shift_from);

        debug_assert!(self.orders[order_index] & 0b1_1111 > flags); // assert that a one has been flipped to 0

        OrderFlags::_get_random_allowed_day(flags, order.frequency, rng)
    }

    pub fn _get_random_allowed_day<R: Rng + ?Sized>(
        flags: u8,
        frequency: Frequency,
        rng: &mut R,
    ) -> Option<DayEnum> {
        match frequency {
            Frequency::None => panic!(
                "Tried to add something with frequency 0 to a route. \
            Frequency 0 is preserved for the dropoff locations"
            ),
            Frequency::Once => {
                if flags == 0 {
                    Some(rng.random())
                } else {
                    None
                }
            }
            Frequency::Twice => match flags {
                0b10000 => Some(DayEnum::Thursday),
                0b01000 => Some(DayEnum::Friday),
                0b00010 => Some(DayEnum::Monday),
                0b00001 => Some(DayEnum::Tuesday),
                0b00100 => panic!("An order with frequency 2 has been put on Wednesday"),
                0b00000 => match rng.random_range(0..4) {
                    0 => Some(DayEnum::Monday),
                    1 => Some(DayEnum::Tuesday),
                    2 => Some(DayEnum::Thursday),
                    3 => Some(DayEnum::Friday),
                    _ => None,
                },
                _ => unreachable!(),
            },
            Frequency::Thrice => {
                let mask = 0b10000 | 0b00100 | 0b00001;
                let available: u8 = mask & !flags;
                if available == 0 {
                    return None;
                }

                // find how many are available
                let mut count = 0;
                for bit in &[0b10000, 0b00100, 0b00001] {
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
                for bit in &[0b10000, 0b01000, 0b00100, 0b00010, 0b00001] {
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
    pub fn day_to_flags(day: DayEnum) -> u8 {
        match day {
            DayEnum::Monday => 0b1_0000,
            DayEnum::Tuesday => 0b0_1000,
            DayEnum::Wednesday => 0b0_0100,
            DayEnum::Thursday => 0b0_0010,
            DayEnum::Friday => 0b0_0001,
        }
    }
    pub fn get_filled_count(&self, order_index: OrderIndex) -> u32 {
        self.orders[order_index].count_ones()
    }
    pub fn get_counts(&self) -> Vec<u32> {
        let mut counts: Vec<u32> = Vec::new();
        for order in &self.orders{
            counts.push(order.count_ones())
        }
        counts
    }
    pub fn clear(&mut self, order_index: OrderIndex) {
        // check if we are actually clearing something here.
        debug_assert_ne!(self.orders[order_index], 0);
        self.orders[order_index] = 0;
    }

    // Give this function an order index and the day you already found an order.
    // This function returns the order days that this order is in a route.
    pub fn get_other_days_of_an_order(&self, order_index: OrderIndex, day_enum: DayEnum) -> Vec<DayEnum>{
        let flags = self.orders[order_index];
        let mut vec = Vec::new();

        for i in 0..5{
            let flag = flags & 1 << i;
            if let Some(day) = Self::flag_to_day(flag) && day != day_enum {
                vec.push(day);
            }
        }

        return vec;

    }
    pub fn flag_to_day(flag: u8) -> Option<DayEnum> {
        match flag{
            0b1_0000 => Some(DayEnum::Monday),
            0b0_1000 => Some(DayEnum::Tuesday),
            0b0_0100 => Some(DayEnum::Wednesday),
            0b0_0010 => Some(DayEnum::Thursday),
            0b0_0001 => Some(DayEnum::Friday),
            _ => None
        }
    }

    pub fn get_flag(&self, order_index: OrderIndex) -> u8 {
        self.orders[order_index]
    }
}

impl Default for OrderFlags{
    fn default() -> Self {
        Self::new(get_orders().len())
    }
}
