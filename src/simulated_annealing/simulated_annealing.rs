use super::placeholder_truck_name_thing::PlaceholderTruckNameThing;
struct SimulatedAnnealing{
    thing1: PlaceholderTruckNameThing,
    thing2: PlaceholderTruckNameThing
    // We could store variables here which are needed for simulated annealing.
}


impl SimulatedAnnealing{
    pub fn new(){
        // intializationthings
    }


    pub fn biiiiiig_loop(&mut self){

        // this ic currently an infinite loop.
        // We will need some predicate to exit this loop
        loop {
            self.do_step();
        }
    }
    fn do_step(&mut self){
        
    }

    fn get_neighborhood(&mut self){

    }
}
