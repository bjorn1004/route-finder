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
    node_i: LVNodeIndex,
    order_i: OrderIndex,
}


impl RemoveRandom {
    pub fn new<R: Rng+?Sized>(truck1: &Week, truck2: &Week, rng: &mut R, order_flags: &OrderFlags) ->  Option<(Self, OrderIndex)>{
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
            return Some((RemoveRandom{
                truck_enum,
                day,
                time_of_day,
                node_i,
                order_i: *order_i,
            }, *order_i));
        }
        unreachable!();
    }
}

impl NeighborMove for RemoveRandom {
    fn evaluate(&self, truck1: &Week, truck2: &Week, order_flags: &OrderFlags) -> Option<CostChange> {
        let route = (if self.truck_enum == TruckEnum::Truck1 {truck1} else {truck2})
            .get(self.day)
            .get(self.time_of_day);
        let order = &get_orders()[self.order_i];

        // make sure we stored the correct order_i
        debug_assert_eq!(self.order_i, *route.linked_vector.get_value(self.node_i).unwrap());

        let time_diff = route.calculate_time_if_remove_node(self.node_i);

        let penalty = if order.frequency as u32 == order_flags.get_filled_count(self.order_i) {
            order.trash() as i32 * 3 * order.frequency as Time
        } else {
          0
        };
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

        #[cfg(debug_assertions)]
        if !route.check_correctness_time("before removing"){
            panic!()
        }

        // if we did all days of an order before remove, we will get a penalty after removing this.
        let penalty = if order.frequency as u32 == order_flags.get_filled_count(self.order_i) {
            order.trash() as i32 * 3 * order.frequency as Time
        } else {
            0
        };

        let time_diff = route.remove_node(self.node_i);
        order_flags.remove_order(self.order_i, self.day);

        // If the route has length 2 after removing a node, the only 2 nodes in the route are dropoffs
        let empty_route = if route.linked_vector.len() == 2{
            -HALF_HOUR
        } else {
            0
        };
        #[cfg(debug_assertions)]
        if !route.check_correctness_time("after removing") {
            panic!()
        }
        println!("did correct");

        time_diff + penalty + empty_route
    }
}
