use rand::Rng;
use crate::simulated_annealing::neighbor_move::add_new_order::AddNewOrder;
use crate::simulated_annealing::neighbor_move::neighbor_move_trait::NeighborMove;
use crate::simulated_annealing::neighbor_move::shift_between_days::ShiftBetweenDays;
use crate::simulated_annealing::neighbor_move::shift_in_route::ShiftInRoute;
use crate::simulated_annealing::simulated_annealing::SimulatedAnnealing;

impl SimulatedAnnealing {
    pub fn choose_neighbor<R: Rng + ?Sized>(&mut self, rng: &mut R) -> Box<dyn NeighborMove> {
        loop {
            let a = rng.random_range(1..4);
            // something to decide which thing to choose
            let transactionthingy: Box<dyn NeighborMove> = match a {
                1 => {
                    if let Some(random_order) = self.unfilled_orders.pop_front() {
                        let new_order = AddNewOrder::new(
                            &self.truck1,
                            &self.truck2,
                            rng,
                            &self.order_flags,
                            random_order,
                        );
                        if new_order.is_none() {
                            self.unfilled_orders.push_front(random_order);
                            continue;
                        }
                        Box::new(new_order.unwrap())
                    } else {
                        continue; // queue is empty, try something else
                    }
                }
                2 => {
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
                3 => {
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
                // remove function, try to remove all days from a single order.
                // for example, if freq==2, remove the order on both the monday and thursday,
                // this will cost O(n) in the length of the routes with our current strurcture
                _ => unreachable!(),
            };
            return transactionthingy;
        }
    }
}