use rand::prelude::SmallRng;
use rand::{Rng, SeedableRng};
use super::transactionoperationnneighborthingidk::nieghbor_move_trait::{Swap2RandomValuesInSameRoute, NeighborMove};
use super::week::Week;
struct SimulatedAnnealing{
    truck1: Week,
    truck2: Week,
    
    // We could store variables here which are needed for simulated annealing.
}


impl SimulatedAnnealing{
    pub fn new(){
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
                1 => { Box::new(Swap2RandomValuesInSameRoute::new())}
                _ => unreachable!(),
            };

            // get the change in capacity/time
            let _ = transactionthingy.evaluate(&self.truck1, &self.truck2);

            // if we want to go through with this thing
            if self.accept(){
                // change the route
                transactionthingy.apply(&mut self.truck1, &mut self.truck2);
                break;
            }
        }
    }

    fn accept(&self) -> bool{
        todo!()
    }
}
