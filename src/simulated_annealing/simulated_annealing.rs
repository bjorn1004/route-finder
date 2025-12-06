use std::collections::VecDeque;
use crate::datastructures::linked_vectors::LinkedVector;
use rand::prelude::{SliceRandom, SmallRng};
use rand::{Rng, SeedableRng};
use crate::datastructures::compact_linked_vector::CompactLinkedVector;
use crate::get_orders;
use crate::resource::Company;
use crate::simulated_annealing::route::OrderIndex;
use crate::simulated_annealing::neighbor_move::neighbor_move_trait::NeighborMove;
use super::neighbor_move::add_new_order::AddNewOrder;
use super::week::Week;
use super::order_day_flags::OrderFlags;

pub struct SimulatedAnnealing{
    truck1: Week,
    truck2: Week,
    order_flags: OrderFlags,
    unfilled_orders: VecDeque<OrderIndex>,
    // We could store variables here which are needed for simulated annealing.
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
pub enum TruckEnum{
    Truck1,
    Truck2
}


impl SimulatedAnnealing{
    pub fn new<R: Rng + ?Sized>(rng: &mut R) -> Self {
        // intializationthings
        let orders = get_orders();
        SimulatedAnnealing{
            truck1: Week::new(),
            truck2: Week::new(),
            order_flags: OrderFlags::new(orders.len()),
            unfilled_orders: Self::fill_unfilled_orders_list(rng),
        }
    }


    pub fn biiiiiig_loop(&mut self){
        let mut rng = SmallRng::seed_from_u64(0);
        // this ic currently an infinite loop.
        // We will need some predicate to exit this loop
        loop {
            self.do_step(&mut rng);
        }
    }

    fn do_step<R: Rng + ?Sized>(&mut self, mut rng: &mut R){
        // not really sure if this is correct
        loop {
            let a = rng.random_range(1..3);
            // something to decide which thing to choose
            let transactionthingy:Box<dyn NeighborMove> = match a {
                1 => {
                    if let Some(random_order) = self.unfilled_orders.pop_front() {
                        Box::new(AddNewOrder::new(&self.truck1, &self.truck2, &mut rng, &self.order_flags, random_order))
                    } else {
                        continue // queue is empty, try something else
                    }
                }
                // remove function, try to remove all days from a single order.
                // for example, if freq==2, remove the order on both the monday and thursday,
                // this will cost O(n) in the length of the routes with our current strurcture
                _ => unreachable!(),
            };

            // get the change in capacity/time

            let cost = transactionthingy.evaluate(&self.truck1, &self.truck2, &self.order_flags);

            // I'm going to use is_none for bad things for now, will later probably be replaced by penalty costs.
            if cost.is_none(){
                continue;
            }
            let cost = cost.unwrap();

            // if we want to go through with this thing
            if self.accept(&transactionthingy){
                // change the route
                transactionthingy.apply(&mut self.truck1, &mut self.truck2, &mut self.order_flags);
                break;
            }
        }
    }

    fn accept(&self, neighbor_move: &Box<dyn NeighborMove>) -> bool{
        true
    }
    
    fn fill_unfilled_orders_list<R: Rng+?Sized>(rng: &mut R) -> VecDeque<OrderIndex>{
        let mut list = Vec::new();
        let orders = get_orders();
        for (index, order) in orders.iter().enumerate(){
            for _ in 0..order.frequency as u8{
                list.push(index);
            }
        }
        list.shuffle(rng);

        VecDeque::from(list)
    }
}
