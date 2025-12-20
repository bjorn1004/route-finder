use std::collections::VecDeque;
use crate::simulated_annealing::order_day_flags::OrderFlags;
use crate::simulated_annealing::route::OrderIndex;
use crate::simulated_annealing::week::Week;

#[derive(Clone)]
pub struct Solution{
    pub truck1: Week,
    pub truck2: Week,
    pub score: i32,
    pub unfilled_orders: VecDeque<OrderIndex>,
    pub order_flags: OrderFlags,
}


impl Solution {
    pub fn fulfilled_order_count(&self) -> usize {
        Self::_fulfilled_order_count(&self.truck1) +
            Self::_fulfilled_order_count(&self.truck2)
    }
    fn _fulfilled_order_count(truck: &Week) -> usize {
        truck.iter().map(|route| route.linked_vector.len() - 2)
            .sum()
    }
}