use rand::Rng;
use crate::datastructures::linked_vectors::{LVNodeIndex, LinkedVector};
use crate::get_orders;
use crate::resource::{Time, HALF_HOUR};
use crate::simulated_annealing::day::TimeOfDay;
use crate::simulated_annealing::neighbor_move::evaluation_helper::{time_between_three_nodes, time_between_two_nodes};
use crate::simulated_annealing::neighbor_move::neighbor_move_trait::{CostChange, NeighborMove, ScoreChange};
use crate::simulated_annealing::order_day_flags::OrderFlags;
use crate::simulated_annealing::route::{OrderIndex, Route};
use crate::simulated_annealing::simulated_annealing::TruckEnum;
use crate::simulated_annealing::week::{DayEnum, Week};

pub struct RemoveRandom {
    truck_enum: TruckEnum,
    day: DayEnum,
    time_of_day: TimeOfDay,
    remove_i: LVNodeIndex,
    order_i: OrderIndex,
}


impl RemoveRandom {
    pub fn new<R: Rng+?Sized>(truck1: &Week, truck2: &Week, rng: &mut R, order_flags: &OrderFlags) ->  Option<Self>{
        let truck_enum: TruckEnum = rng.random();
        let day:DayEnum = rng.random();
        let time_of_day:TimeOfDay = rng.random();
        let route = (if truck_enum == TruckEnum::Truck1 {truck1} else {truck2}).get(day).get(time_of_day);
        let lv = &route.linked_vector;
        if lv.len() == 2 {
            return None;
        }
        while let Some((node_i, order_i)) = lv.get_random(rng){
            if node_i == lv.get_head_index()? || node_i == lv.get_tail_index()?{
                continue;
            }
            let orders = get_orders();
            if orders[*order_i].frequency as u8 !=  1{
                // we only do orders with frequency 1 for now.
                return None;
            }

            return Some(RemoveRandom{
                truck_enum,
                day,
                time_of_day,
                remove_i: node_i,
                order_i: *order_i,
            });
        }
        unreachable!();
    }
    fn time_difference(&self, route: &Route) -> Time {
        let lv = &route.linked_vector;
        let orders = get_orders();

        let back = orders[*lv.get_prev_value_unsafe(self.remove_i)].matrix_id;
        let current = orders[self.order_i].matrix_id;
        let front = orders[*lv.get_next_value_unsafe(self.remove_i)].matrix_id;

        let removed_time = -time_between_three_nodes(back, current, front);
        let added_time = time_between_two_nodes(back, front);

        removed_time + added_time
    }
}

impl NeighborMove for RemoveRandom {
    fn evaluate(&self, truck1: &Week, truck2: &Week, order_flags: &OrderFlags) -> Option<CostChange> {
        let route = (if self.truck_enum == TruckEnum::Truck1 {truck1} else {truck2})
            .get(self.day)
            .get(self.time_of_day);
        let orders = get_orders();

        // make sure we stored the correct order_i
        #[cfg(debug_assertions)]
        assert_eq!(self.order_i, *route.linked_vector.get_value(self.remove_i).unwrap());

        let time_diff = self.time_difference(route);

        // for now, frequency will always be one.
        let penalty = orders[self.order_i].trash() as i32 * 3 * orders[self.order_i].frequency as Time;
        let empty_route = if route.linked_vector.len() == 3{
            -HALF_HOUR
        } else {
            0
        };

        Some(time_diff + penalty + empty_route)
    }

    fn apply(&self, truck1: &mut Week, truck2: &mut Week, order_flags: &mut OrderFlags) -> ScoreChange {
        let route = (if self.truck_enum == TruckEnum::Truck1 {truck1} else {truck2})
            .get_mut(self.day)
            .get_mut(self.time_of_day);
        let order = &get_orders()[self.order_i];

        let time_diff = self.time_difference(route);


        let lv = &mut route.linked_vector;

        lv.remove(self.remove_i);
        route.time += time_diff;
        route.capacity -= order.trash();
        order_flags.remove_order(self.order_i, self.day);

        // for now, frequency will always be one.
        let penalty = order.trash() as i32 * 3 * order.frequency as Time;
        let empty_route = if route.linked_vector.len() == 3{
            -HALF_HOUR
        } else {
            0
        };
        println!("deleted yay");
        time_diff + penalty + empty_route
    }
}
