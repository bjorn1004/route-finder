use rand::Rng;
use crate::simulated_annealing::neighbor_move::add_new_order::AddNewOrder;
use crate::simulated_annealing::neighbor_move::neighbor_move_trait::NeighborMove;
use crate::simulated_annealing::neighbor_move::shift_between_days::ShiftBetweenDays;
use crate::simulated_annealing::neighbor_move::shift_in_route::ShiftInRoute;
use crate::simulated_annealing::simulated_annealing::SimulatedAnnealing;
use rand::distr::weighted::WeightedIndex;
use rand::prelude::*;
use crate::resource::MINUTE;
use crate::simulated_annealing::neighbor_move::remove::RemoveOrder;
use crate::simulated_annealing::route::OrderIndex;

impl SimulatedAnnealing {
    pub fn choose_neighbor<R: Rng + ?Sized>(&mut self, rng: &mut R) -> (Box<dyn NeighborMove>, Option<OrderIndex>) {
        // https://docs.rs/rand_distr/latest/rand_distr/weighted/struct.WeightedIndex.html
        let weights = [
            100000, // add new order
            100000, // shift inside of a route
            100000, // shift between days
            if self.score <= 6000*MINUTE {1} else {0}, // remove
        ];
        let weights = WeightedIndex::new(&weights).unwrap();
        let mut order_to_add:Option<OrderIndex> = None;
        loop {
            let a = weights.sample(rng);

            // something to decide which thing to choose
            let transactionthingy: Box<dyn NeighborMove> = match a {
                0 => {
                    if let Some(random_order) = self.unfilled_orders.pop_front() {
                        let new_order = AddNewOrder::new(
                            &self.truck1,
                            &self.truck2,
                            rng,
                            &self.order_flags,
                            random_order,
                        );
                        if new_order.is_none() {
                            self.unfilled_orders.push_back(random_order);
                            continue;
                        }
                        Box::new(new_order.unwrap())
                    } else {
                        continue; // queue is empty, try something else
                    }
                }
                1 => {
                    let shift = ShiftInRoute::new(
                        &self.truck1,
                        &self.truck2,
                        rng
                    );
                    if shift.is_none() {
                        continue;
                    }
                    Box::new(shift.unwrap())
                }
                2 => {
                    let shift = ShiftBetweenDays::new(
                        &self.truck1,
                        &self.truck2,
                        rng,
                        &self.order_flags,
                    );
                    if shift.is_none() {
                        continue;
                    }
                    Box::new(shift.unwrap())
                }
                3 => {
                    if let Some((remove, _order_to_add)) = RemoveOrder::new(
                        &self.truck1,
                        &self.truck2,
                        rng
                    ){
                        order_to_add = Some(_order_to_add);
                        Box::new(remove)
                    } else {
                        continue;
                    }
                }
                // remove function, try to remove all days from a single order.
                // for example, if freq==2, remove the order on both the monday and thursday,
                // this will cost O(n) in the length of the routes with our current strurcture
                _ => unreachable!(),
            };
            return (transactionthingy, order_to_add);
        }
    }
}