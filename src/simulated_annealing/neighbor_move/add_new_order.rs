use petgraph::matrix_graph::NodeIndex;
use petgraph::visit::NodeIndexable;
use rand::Rng;
use crate::datastructures::linked_vectors::{LinkedVector, LVNodeIndex};
use crate::{get_distance_matrix, get_orders};
use crate::simulated_annealing::day::TimeOfDay;
use crate::simulated_annealing::neighbor_move::evaluation_helper::{time_between_three_nodes, time_between_two_nodes};
use crate::simulated_annealing::order_day_flags::OrderFlags;
use crate::simulated_annealing::route::OrderIndex;
use crate::simulated_annealing::neighbor_move::neighbor_move_trait::{CostChange, NeighborMove};
use crate::simulated_annealing::week::{DayEnum, Week};

/// This will add an order to a random route where it is allowed to add it to.
/// If you try to add an order, that doesn't have any allowed routes, it panics
pub struct AddNewOrder {
    is_truck_1: bool,
    day: DayEnum,
    time_of_day: TimeOfDay,
    insert_after_index: LVNodeIndex,
    order: OrderIndex,
}
impl AddNewOrder {
    pub fn new<R: Rng+?Sized>(truck1: &Week, truck2: &Week, rng: &mut R, order_flags: &OrderFlags, order: OrderIndex) ->  Option<Self>{

        let is_truck_1:bool = rng.random();
        let truck = if is_truck_1 {truck1} else {truck2};

        let capacity = get_orders()[order].trash();
        // check if there is still an allowed day open
        if let Some(day_enum) = order_flags.get_random_allowed_day(order, rng){
            let day = truck.get(day_enum);
            let (route, time_of_day_enum) = day.get_random(rng);
            if route.capacity + capacity > 100_000{
                return None;
            }

            let lv = &route.linked_vector;
            while let Some((index, _)) = lv.get_random(rng) {
                if lv.get_tail_index() == Some(index) {
                    // we don't want to add behind the tail.
                    // we could try to instead insert in front of the tail but don't want to try that rn.
                    // instead we try to randomly find a new value which hopefully isn't the tail
                    continue
                }


                return Some(AddNewOrder {
                    is_truck_1,
                    day: day_enum,
                    time_of_day: time_of_day_enum,
                    insert_after_index: index,
                    order,
                })
            }
            panic!("how did we get here?")
        } else {
            panic!("We tried to add an order, but there are no days days left where this order could be added")
        }
    }


    fn calculate_time_difference(&self, truck1: &Week, truck2: &Week) -> f32{
        let orders = get_orders();
        let order = &orders[self.order];
        let route = (if self.is_truck_1 { truck1 } else {truck2}).get(self.day).get(self.time_of_day);

        let before_order_i = *route.linked_vector.get_value(self.insert_after_index).unwrap();
        let after_order_i = *route.linked_vector.get_next_value(self.insert_after_index).unwrap();

        let dist = get_distance_matrix();

        let before: NodeIndex<u16> = orders[before_order_i].matrix_id.into();
        let after: NodeIndex<u16> = orders[after_order_i].matrix_id.into();
        let middle: NodeIndex<u16> = orders[self.order].matrix_id.into();

        // let before = dist.from_index(orders[before_order_i].matrix_id as usize);
        // let after = dist.from_index(orders[after_order_i].matrix_id as usize);
        // let middle = dist.from_index(orders[self.order].matrix_id as usize);

        let old_time = time_between_two_nodes(before, after);

        let new_time = time_between_three_nodes(before, middle, after);

        // als totale reistijd > toegestane reistijd

        new_time - old_time + order.emptying_time
    }
}


impl NeighborMove for AddNewOrder {
    fn evaluate(&self, truck1: &Week, truck2: &Week, order_flags: &OrderFlags) -> Option<CostChange>{
        let orders = get_orders();
        let order = &orders[self.order];
        let mut cost: f32 = 0f32;

        // stel dit is de laatste van een order, 3x ledigingsduur weghalen
        if order_flags.get_filled_count(self.order) + 1 == order.frequency as u32{
            cost -= 3f32 * order.emptying_time * order.frequency as u32 as f32;
        }

        let time = self.calculate_time_difference(truck1, truck2);



        let a = (if self.is_truck_1 {truck1} else {truck2}).get(self.day);
        if a.get_time() + time > 12f32*60f32*60f32{
            // return None;
        }

        Some(cost + time)
    }

    fn apply(&self, truck1: &mut Week, truck2: &mut Week, order_flags: &mut OrderFlags) {
        order_flags.add_order(self.order, self.day);

        let time_difference = self.calculate_time_difference(&truck1, &truck2);
        let truck = if self.is_truck_1 {truck1} else {truck2};
        let day = truck.get_mut(self.day);
        let route = day.get_mut(self.time_of_day);
        route.linked_vector.insert_after(self.insert_after_index, self.order);
        route.capacity += get_orders()[self.order].trash();
        route.time += time_difference;
        route.check_correctness_time();
    }
}