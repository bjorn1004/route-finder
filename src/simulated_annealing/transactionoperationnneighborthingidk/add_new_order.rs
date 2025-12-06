use rand::Rng;
use crate::datastructures::linked_vectors::{LinkedVector, NodeIndex};
use crate::get_orders;
use crate::resource::Frequency;
use crate::simulated_annealing::day::TimeOfDay;
use crate::simulated_annealing::order_day_flags::OrderFlags;
use crate::simulated_annealing::route::OrderIndex;
use crate::simulated_annealing::transactionoperationnneighborthingidk::add_new_order::AddNewOrderEnum::NoOrderToAdd;
use crate::simulated_annealing::transactionoperationnneighborthingidk::nieghbor_move_trait::NeighborMove;
use crate::simulated_annealing::week::{DayEnum, Week};

pub struct AddNewOrder{
    operation: AddNewOrderEnum
}

pub enum AddNewOrderEnum{
    NoOrderToAdd(),
    AddOrder{
        is_truck_1: bool,
        day: DayEnum,
        time_of_day: TimeOfDay,
        index: NodeIndex,
        order: OrderIndex
    }
}
impl AddNewOrder{
    pub fn new<R: Rng+?Sized>(truck1: &Week, truck2: &Week, rng: &mut R, order_flags: &OrderFlags, order: Option<OrderIndex>) ->  Self{
        if let Some(order) = order{
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
                        continue
                    }

                    return AddNewOrder {
                        operation: AddNewOrderEnum::AddOrder {
                            is_truck_1,
                            day: day_enum,
                            time_of_day: time_of_day_enum,
                            index,
                            order
                        }
                    }
                }
            } else {
                panic!("We tried to add an order, but there are no days days left where this order could be added")
            }
        }
        AddNewOrder{
            operation: NoOrderToAdd()
        }
    }
}


impl NeighborMove for AddNewOrder{
    fn evaluate(&self, truck1: &Week, truck2: &Week) {
        // return error value? or a bad value?
        todo!()
    }

    fn apply(&self, truck1: &mut Week, truck2: &mut Week, order_flags: &mut OrderFlags) {
        if let NoOrderToAdd() = self.operation {
            return
        } else if let AddNewOrderEnum::AddOrder {
            is_truck_1,
            day,
            time_of_day,
            index,
            order,
        } = self.operation {
            order_flags.add_order(order, day);

            let truck = if is_truck_1 {truck1} else {truck2};
            let day = truck.get_mut(&day);
            let route = day.get_mut(time_of_day);
            route.linked_vector.insert_after(index, order);

        }
    }
}