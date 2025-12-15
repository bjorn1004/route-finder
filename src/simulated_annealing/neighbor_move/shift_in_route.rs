use crate::datastructures::linked_vectors::{LVNodeIndex, LinkedVector};
use crate::get_orders;
use crate::resource::Time;
use crate::simulated_annealing::day::TimeOfDay;
use crate::simulated_annealing::neighbor_move::evaluation_helper::{
    time_between_three_nodes, time_between_two_nodes,
};
use crate::simulated_annealing::neighbor_move::neighbor_move_trait::{CostChange, NeighborMove};
use crate::simulated_annealing::order_day_flags::OrderFlags;
use crate::simulated_annealing::week::{DayEnum, Week};
use rand::Rng;

pub struct ShiftInRoute {
    is_truck1: bool,
    day: DayEnum,
    time_of_day: TimeOfDay,
    shifting_node: LVNodeIndex,
    target_neighbor1: LVNodeIndex,
    target_neighbor2: LVNodeIndex,
}

impl ShiftInRoute {
    pub fn new<R: Rng + ?Sized>(truck1: &Week, truck2: &Week, rng: &mut R) -> Option<Self> {
        let is_truck1: bool = rng.random();
        let truck = if is_truck1 { truck1 } else { truck2 };

        let day_enum: DayEnum = rng.random();
        let day = truck.get(day_enum);

        let time_of_day: TimeOfDay = rng.random();
        let route = day.get(time_of_day);

        let lv = &route.linked_vector;
        let shifting_node: LVNodeIndex;
        if lv.len() < 5 {
            return None;
        }
        loop {
            let (node_index, _) = lv.get_random(rng).unwrap();
            if node_index == lv.get_head_index().unwrap()
                || node_index == lv.get_tail_index().unwrap()
            {
                continue;
            }
            shifting_node = node_index;
            break;
        }

        let before_shifting_node = lv.get_prev_index(shifting_node).unwrap();

        let target_neighbor1: LVNodeIndex;
        loop {
            let (node_index, _) = lv.get_random(rng).unwrap();
            if node_index == shifting_node
                || node_index == before_shifting_node
                || node_index == lv.get_tail_index().unwrap()
            {
                continue;
            }
            target_neighbor1 = node_index;
            break;
        }

        let target_neighbor2: LVNodeIndex = lv.get_next_index(target_neighbor1).unwrap();

        debug_assert_eq!(lv.get_value(target_neighbor2).unwrap(), lv.get_next_value_unsafe(target_neighbor1));
        Some(ShiftInRoute {
            is_truck1,
            day: day_enum,
            time_of_day,
            shifting_node,
            target_neighbor1,
            target_neighbor2,
        })
    }

    pub fn time_difference(&self, truck1: &Week, truck2: &Week) -> Time {

        let truck = if self.is_truck1 { truck1 } else { truck2 };
        let route = truck.get(self.day).get(self.time_of_day);
        let lv = &route.linked_vector;

        let time_difference =
            route.calculate_remove_node(self.shifting_node) +
                route.calculate_add_order(self.target_neighbor1, *lv.get_value_unsafe(self.shifting_node));

        #[cfg(debug_assertions)]
        {
            // This is the old way I calaculated it. I have now abstracted the code into the route struct.
            // I'll leave this here for now. I now for sure that the code below is correct.

            let orders = get_orders();

            let before_shift = orders[*lv.get_prev_value(self.shifting_node).unwrap()].matrix_id;
            let after_shift = orders[*lv.get_next_value(self.shifting_node).unwrap()].matrix_id;

            let t1 = orders[*lv.get_value_unsafe(self.target_neighbor1)].matrix_id;
            let shift = orders[*lv.get_value_unsafe(self.shifting_node)].matrix_id;
            let t2 = orders[*lv.get_value_unsafe(self.target_neighbor2)].matrix_id;

            debug_assert_eq!(orders[*lv.get_next_value_unsafe(self.target_neighbor1)].matrix_id, t2);

            // add the difference between the shifting_node and the two nodes where it will be put between
            let mut old_time_difference = time_between_three_nodes(t1, shift, t2);
            // remove the time between these two nodes
            old_time_difference -= time_between_two_nodes(t1, t2);

            // add the time between the two neighbors of the node that will be shifted
            old_time_difference += time_between_two_nodes(before_shift, after_shift);
            // remove the time between the node that will be shifted and it's current neighbors
            old_time_difference -= time_between_three_nodes(before_shift, shift, after_shift);

            // normally we would calculate the emptying time here,
            // but the shifted node will end up in this route again,
            // thus we will end up with the same emptying time for this route.
            assert_eq!(old_time_difference, time_difference);
        }

        time_difference
    }
}
impl NeighborMove for ShiftInRoute {
    fn evaluate(&self, truck1: &Week, truck2: &Week, _: &OrderFlags) -> Option<CostChange> {
        Some(self.time_difference(truck1, truck2))
    }

    fn apply(&self, truck1: &mut Week, truck2: &mut Week, _: &mut OrderFlags) -> Time {
        // calculate the change in time after this operation
        let time_difference = self.time_difference(truck1, truck2);

        let truck = if self.is_truck1 { truck1 } else { truck2 };
        let route = truck.get_mut(self.day).get_mut(self.time_of_day);

        route.time += time_difference;

        // move the shifting_node in the lv
        let lv = &mut route.linked_vector;
        let shifting_value = *lv.get_value(self.shifting_node).unwrap();
        lv.remove(self.shifting_node);
        lv.insert_after(self.target_neighbor1, shifting_value);
        // don't need to compact, because the lv has the same length as before the operations.

        route.check_correctness_time();
        time_difference
    }
}
