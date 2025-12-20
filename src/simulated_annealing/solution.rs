use std::collections::VecDeque;
use std::fs::{read_to_string, File};
use rand::Rng;
use crate::{get_orders, MULTIPL_ADD_AND_REMOVE};
use crate::resource::Company;
use crate::simulated_annealing::day::Day;
use crate::simulated_annealing::order_day_flags::OrderFlags;
use crate::simulated_annealing::route::OrderIndex;
use crate::simulated_annealing::score_calculator::calculate_starting_score;
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
    pub fn new() -> Self {
        Solution {
            truck1: Default::default(),
            truck2: Default::default(),
            score: calculate_starting_score(),
            unfilled_orders: Self::fill_unfilled_orders_list(),
            order_flags: Default::default(),
        }
    }

    fn fill_unfilled_orders_list() -> VecDeque<OrderIndex> {
    let mut deliveries = Vec::new();
    let orders = get_orders();
    if MULTIPL_ADD_AND_REMOVE{
    for i in 0..orders.len() - 1{
    deliveries.push(i);
    }
    VecDeque::from(deliveries)
    } else {
    let mut list: Vec<(usize, &Company)> = orders.iter().enumerate().collect();
    list.sort_by_key(|(_, order)| order.frequency as u8);
    for (index, order) in list.iter() {
    for _ in 0..order.frequency as u8 {
    deliveries.push(*index);
    }
    }

    VecDeque::from(deliveries)

    }
    }


    pub fn fulfilled_order_count(&self) -> usize {
        Self::_fulfilled_order_count(&self.truck1) +
            Self::_fulfilled_order_count(&self.truck2)
    }
    fn _fulfilled_order_count(truck: &Week) -> usize {
        truck.iter().map(|route| route.linked_vector.len() - 2)
            .sum()
    }

    pub fn from_file(path: &str) -> Solution{
        let solution_file = read_to_string(path)
            .expect("Could not read the solution file");
        let lines = solution_file.lines();
        let solution = Self::new();



        todo!()
    }
}

impl Default for Solution {
    fn default() -> Self {
        Self::new()
    }
}