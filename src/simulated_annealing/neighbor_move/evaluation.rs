use std::iter::Sum;
use std::ops::Add;
use crate::resource::Time;
use crate::simulated_annealing::neighbor_move::neighbor_move_trait::CostChange;

#[derive(Default)]
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
impl Sum for Evaluation {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::default(), |a, b| a + b)
    }
}
