use crate::resource::Time;
use crate::simulated_annealing::solution::Solution;
use crate::simulated_annealing::neighbor_move::evaluation::Evaluation;

pub trait NeighborMove {
    // this would return the difference in volume or time
    // (not sure how to implement this yet)
    fn evaluate(&self, solution: &Solution) -> Evaluation;
    // this would perform the thing on the schedules.
    fn apply(&self, solution: &mut Solution) -> ScoreChange;
}

pub type CostChange = Time;
pub type ScoreChange = Time;
