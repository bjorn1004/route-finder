use rand::{Rng};
use crate::simulated_annealing::neighbor_move::neighbor_move_trait::NeighborMove;
use crate::simulated_annealing::neighbor_move::shift_between_days::ShiftBetweenDays;
use crate::simulated_annealing::neighbor_move::shift_in_route::ShiftInRoute;
use crate::simulated_annealing::simulated_annealing::{EndOfStepInfo, SimulatedAnnealing};
use rand::distr::weighted::WeightedIndex;
use rand::prelude::*;
use crate::simulated_annealing::neighbor_move::add_multiple_at_once::AddMultipleNewOrders;
use crate::simulated_annealing::neighbor_move::remove_multiple_at_once::RemoveMultipleOrders;
use crate::simulated_annealing::solution::Solution;

impl SimulatedAnnealing {
    pub fn choose_neighbor<R: Rng + ?Sized>(&mut self, rng: &mut R, weights: [i32;4], solution: &mut Solution) -> (Box<dyn NeighborMove>, EndOfStepInfo) {
        // https://docs.rs/rand_distr/latest/rand_distr/weighted/struct.WeightedIndex.html
        let weights = WeightedIndex::new(weights).unwrap();
        let mut order_to_add:EndOfStepInfo = EndOfStepInfo::Nothing;
        loop {
            let a = weights.sample(rng);

            // something to decide which thing to choose
            let transactionthingy: Box<dyn NeighborMove> = match a {
                0 => {
                    if let Some(random_order) = solution.unfilled_orders.pop_front() {
                        let new_order = AddMultipleNewOrders::new(
                            solution,
                            rng,
                            random_order);
                        if new_order.is_none() {
                            solution.unfilled_orders.push_back(random_order);
                            continue;
                        }
                        order_to_add = EndOfStepInfo::Add(random_order);
                        Box::new(new_order.unwrap())
                    } else {
                        continue;
                    }
                }
                1 => {
                    let shift = ShiftInRoute::new(
                        solution,
                        rng
                    );
                    if shift.is_none() {
                        continue;
                    }
                    Box::new(shift.unwrap())
                }
                2 => {
                    let shift = ShiftBetweenDays::new(
                        solution,
                        rng,
                    );
                    if shift.is_none() {
                        continue;
                    }
                    Box::new(shift.unwrap())
                }
                3 => {
                    if let Some((remove, _order_to_add)) = RemoveMultipleOrders::new(
                        solution,
                        rng,
                    ){
                        order_to_add = EndOfStepInfo::Removed(_order_to_add);
                        Box::new(remove)
                    } else {
                        continue;
                    }
                }
                _ => unreachable!(),
            };
            return (transactionthingy, order_to_add);
        }
    }
}