use rand::prelude::SmallRng;
use rand::{Rng, SeedableRng};
use crate::resource::Company;
use super::transactionoperationnneighborthingidk::nieghbor_move_trait::{Swap2RandomValuesInSameRoute, NeighborMove};
use super::week::Week;
use super::order_day_flags::OrderFlags;

struct SimulatedAnnealing{
    truck1: Week,
    truck2: Week,
    order_flags: OrderFlags
    
    // We could store variables here which are needed for simulated annealing.
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
pub enum TruckEnum{
    Truck1,
    Truck2
}


impl SimulatedAnnealing{
    pub fn new(orders: Vec<Company>) -> Self {
        SimulatedAnnealing{
            truck1: Week::new(),
            truck2: Week::new(),
            order_flags: OrderFlags::new(orders.len()),
        }
        // intializationthings
    }

    pub fn biiiiiig_loop(&mut self){
        let mut rng = SmallRng::seed_from_u64(0);
        // this ic currently an infinite loop.
        // We will need some predicate to exit this loop
        loop {
            self.do_step(&mut rng);
        }
    }

    fn do_step(&mut self, rng: &mut SmallRng){
        // not really sure if this is correct
        loop {
            let a = rng.random_range(1..3);
            // something to decide which thing to choose
            let transactionthingy:Box<dyn NeighborMove> = match a {
                1 => { Box::new(Swap2RandomValuesInSameRoute::new(&self.truck1, &self.truck2, &self.order_flags))}
                _ => unreachable!(),
            };

            // get the change in capacity/time
            let _ = transactionthingy.evaluate(&self.truck1, &self.truck2);

            // if we want to go through with this thing
            if self.accept(){
                // change the route
                transactionthingy.apply(&mut self.truck1, &mut self.truck2, &self.order_flags);
                break;
            }
        }
    }

    fn accept(&self) -> bool{
        todo!()
    }
}
