use crate::simulated_annealing::week::Week;
use crate::simulated_annealing::score_calculator::{calculate_score};

#[derive(Clone)]
pub struct Solution{
    pub truck1: Week,
    pub truck2: Week,
    pub score: i32,
}