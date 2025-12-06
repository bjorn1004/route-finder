use rand::Rng;
use crate::datastructures::linked_vectors::{LinkedVector, NodeIndex};
use crate::get_orders;
use crate::resource::Frequency;
use crate::simulated_annealing::day::TimeOfDay;
use crate::simulated_annealing::order_day_flags::OrderFlags;
use crate::simulated_annealing::route::OrderIndex;
use crate::simulated_annealing::transactionoperationnneighborthingidk::nieghbor_move_trait::NeighborMove;
use crate::simulated_annealing::week::{DayEnum, Week};

/// This will add an order to a random route where it is allowed to add it to.
/// If you try to add an order, that doesn't have any allowed routes, it panics
pub struct AddNewOrder{
    is_truck_1: bool,
    day: DayEnum,
    time_of_day: TimeOfDay,
    index: NodeIndex,
    order: OrderIndex
}
impl AddNewOrder{
    pub fn new<R: Rng+?Sized>(truck1: &Week, truck2: &Week, rng: &mut R, order_flags: &OrderFlags, order: OrderIndex) ->  Self{

        let is_truck_1:bool = rng.random();
        let truck = if is_truck_1 {truck1} else {truck2};

        // check if there is still an allowed day open
        if let Some(day_enum) = order_flags.get_random_allowed_day(order, rng){
            let day = truck.get(&day_enum);
            let (route, time_of_day_enum) = day.get_random(rng);
            let lv = &route.linked_vector;
            while let Some((index, orderIndex)) = lv.get_random(rng) {
                if lv.get_tail_index() == Some(index) {
                    // we don't want to add behind the tail.
                    // we could try to instead insert in front of the tail but don't want to try that rn.
                    // instead we try to randomly find a new value which hopefully isn't the tail
                    continue
                }

                return AddNewOrder {
                    is_truck_1,
                    day: day_enum,
                    time_of_day: time_of_day_enum,
                    index,
                    order
                }
            }
            panic!("how did we get here?")
        } else {
            panic!("We tried to add an order, but there are no days days left where this order could be added")
        }
    }
}


impl NeighborMove for AddNewOrder{
    fn evaluate(&self, truck1: &Week, truck2: &Week) {
        // return error value? or a bad value?
        todo!()
    }

    fn apply(&self, truck1: &mut Week, truck2: &mut Week, order_flags: &mut OrderFlags) {
        order_flags.add_order(self.order, self.day);

        let truck = if self.is_truck_1 {truck1} else {truck2};
        let day = truck.get_mut(&self.day);
        let route = day.get_mut(self.time_of_day);
        route.linked_vector.insert_after(self.index, self.order);
    }
}