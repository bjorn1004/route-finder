use std::iter::Sum;
use std::ops::Add;
use crate::resource::Time;
use crate::simulated_annealing::solution::Solution;

pub trait NeighborMove {
    // this would return the difference in volume or time
    // (not sure how to implement this yet)
    fn evaluate(&self, solution: &Solution) -> Evaluation;
    // this would perform the thing on the schedules.
    fn apply(&self, solution: &mut Solution) -> ScoreChange;
}

pub type CostChange = Time;
pub type ScoreChange = Time;
pub struct Evaluation{
    pub cost: CostChange,
    // ben nog op zoek naar betere namen
    pub time_overflow_delta: Time,
    pub capacity_overflow_delta: i32,
}

impl Add for Evaluation {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            cost: self.cost + other.cost,
            time_overflow_delta: self.time_overflow_delta + other.time_overflow_delta,
            capacity_overflow_delta: self.capacity_overflow_delta
                + other.capacity_overflow_delta,
        }
    }
}
impl Default for Evaluation {
    fn default() -> Self {
        Self {
            cost: 0,
            time_overflow_delta: 0,
            capacity_overflow_delta: 0,
        }
    }
}
impl Sum for Evaluation {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::default(), |a, b| a + b)
    }
}
