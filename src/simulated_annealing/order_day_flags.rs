use crate::simulated_annealing::route::OrderIndex;
use super::week::DayEnum;
pub struct OrderFlags {
    orders: Vec<u8>,
}


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
}