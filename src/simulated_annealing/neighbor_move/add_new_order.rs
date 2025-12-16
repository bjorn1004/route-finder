use rand::Rng;
use crate::datastructures::linked_vectors::{LinkedVector, LVNodeIndex};
use crate::{get_orders};
use crate::resource::{Time, HALF_HOUR};
use crate::simulated_annealing::day::TimeOfDay;
use crate::simulated_annealing::neighbor_move::evaluation_helper::{time_between_three_nodes, time_between_two_nodes};
use crate::simulated_annealing::order_day_flags::OrderFlags;
use crate::simulated_annealing::route::OrderIndex;
use crate::simulated_annealing::neighbor_move::neighbor_move_trait::{CostChange, NeighborMove, ScoreChange};
use crate::simulated_annealing::simulated_annealing::TruckEnum;
use crate::simulated_annealing::week::{DayEnum, Week};

/// This will add an order to a random route where it is allowed to add it to.
/// If you try to add an order, that doesn't have any allowed routes, it panics
pub struct AddNewOrder {
    truck_enum: TruckEnum,
    day: DayEnum,
    time_of_day: TimeOfDay,
    insert_after_index: LVNodeIndex,
    order: OrderIndex,
}
impl AddNewOrder {
    pub fn new<R: Rng+?Sized>(truck1: &Week, truck2: &Week, rng: &mut R, order_flags: &OrderFlags, order: OrderIndex) ->  Option<Self>{

        let truck_enum: TruckEnum = rng.random();
        let truck = if truck_enum == TruckEnum::Truck1 {truck1} else {truck2};

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
                    truck_enum,
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


    #[cfg(debug_assertions)]
    fn calculate_time_difference(&self, truck1: &Week, truck2: &Week) -> Time{
        let orders = get_orders();
        let order = &orders[self.order];
        let route = (if self.truck_enum == TruckEnum::Truck1 { truck1 } else {truck2}).get(self.day).get(self.time_of_day);

        let before_order_i = *route.linked_vector.get_value_unsafe(self.insert_after_index);
        let after_order_i = *route.linked_vector.get_next_value_unsafe(self.insert_after_index);

        let before = orders[before_order_i].matrix_id;
        let after = orders[after_order_i].matrix_id;
        let middle = orders[self.order].matrix_id;

        let old_time = time_between_two_nodes(before, after);

        let new_time = time_between_three_nodes(before, middle, after);

        new_time - old_time + order.emptying_time
    }
}


impl NeighborMove for AddNewOrder {
    fn evaluate(&self, truck1: &Week, truck2: &Week, order_flags: &OrderFlags) -> CostChange {

        let route = (if self.truck_enum == TruckEnum::Truck1 { truck1 } else { truck2 }).get(self.day).get(self.time_of_day);
        let time = route.calculate_add_order(self.insert_after_index, self.order);

        let order = &get_orders()[self.order];

        // stel dit is de laatste van een order, 3x ledigingsduur weghalen
        let penalty = if order_flags.get_filled_count(self.order) + 1 == order.frequency as u32{
            -order.penalty()
        } else {
            0
        };

        #[cfg(debug_assertions)]
        {
            let old_time_calaculator = self.calculate_time_difference(truck1, truck2);
            assert_eq!(time, old_time_calaculator);
        }

        penalty + time
    }

    fn apply(&self, truck1: &mut Week, truck2: &mut Week, order_flags: &mut OrderFlags) -> ScoreChange {
        #[cfg(debug_assertions)]
        let checker_time_diff = self.calculate_time_difference(truck1, truck2);

        let route= (if self.truck_enum == TruckEnum::Truck1 {truck1} else {truck2})
            .get_mut(self.day)
            .get_mut(self.time_of_day);

        let time_difference = route.apply_add_order(self.insert_after_index, self.order);
        order_flags.add_order(self.order, self.day);
        #[cfg(debug_assertions)]
        assert_eq!(time_difference, checker_time_diff);

        let order = &get_orders()[self.order];

        // stel dit is de laatste van een order, 3x ledigingsduur weghalen
        let penalty = if order_flags.get_filled_count(self.order) == order.frequency as u32{
            -3 * order.emptying_time * order.frequency as Time
        } else {
            0
        };

        let new_route_added = if route.linked_vector.len() == 3{
            HALF_HOUR
        } else {
            0
        };
        time_difference + penalty + new_route_added
    }
}