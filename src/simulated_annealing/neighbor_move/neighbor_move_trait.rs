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
    pub time_overflow: Time,
    pub time_overflow_lessened: Time,
    pub capacity_overflow: i32,
    pub capacity_overflow_lessened: i32,
}

impl Add for Evaluation {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            cost: self.cost + other.cost,
            time_overflow: self.time_overflow + other.time_overflow,
            time_overflow_lessened: self.time_overflow_lessened + other.time_overflow_lessened,
            capacity_overflow: self.capacity_overflow + other.capacity_overflow,
            capacity_overflow_lessened: self.capacity_overflow_lessened
                + other.capacity_overflow_lessened,
        }
    }
}
impl Default for Evaluation {
    fn default() -> Self {
        Self {
            cost: CostChange::default(),
            time_overflow: Time::default(),
            time_overflow_lessened: Time::default(),
            capacity_overflow: 0,
            capacity_overflow_lessened: 0,
        }
    }
}
impl Sum for Evaluation {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::default(), |a, b| a + b)
    }
}
impl Evaluation {
    pub fn validate(self) -> Self {
        debug_assert!(self.time_overflow >= 0);
        debug_assert!(self.time_overflow_lessened >= 0);
        debug_assert!(self.capacity_overflow >= 0);
        debug_assert!(self.capacity_overflow_lessened >= 0);
        self
    }
}
