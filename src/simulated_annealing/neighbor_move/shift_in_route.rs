use rand::Rng;
use crate::datastructures::linked_vectors::{LinkedVector, LVNodeIndex};
use crate::{get_orders};
use crate::resource::Time;
use crate::simulated_annealing::day::TimeOfDay;
use crate::simulated_annealing::neighbor_move::evaluation_helper::{time_between_three_nodes, time_between_two_nodes};
use crate::simulated_annealing::neighbor_move::neighbor_move_trait::{CostChange, NeighborMove};
use crate::simulated_annealing::order_day_flags::OrderFlags;
use crate::simulated_annealing::week::{DayEnum, Week};

pub struct ShiftInRoute{
    is_truck1: bool,
    day: DayEnum,
    time_of_day: TimeOfDay,
    shifting_node: LVNodeIndex,
    target_neighbor1: LVNodeIndex,
    target_neighbor2: LVNodeIndex,
}

impl ShiftInRoute{
    pub fn new<R: Rng+?Sized>(truck1: &Week, truck2: &Week, rng: &mut R) ->  Option<Self> {
        let is_truck1:bool = rng.random();
        let truck = if is_truck1 {truck1} else {truck2};

        let day_enum:DayEnum = rng.random();
        let day = truck.get(day_enum);

        let time_of_day:TimeOfDay = rng.random();
        let route = day.get(time_of_day);

        let lv = &route.linked_vector;
        let shifting_node: LVNodeIndex;
        if lv.len() < 5{
            return None;
        }
        loop {
            let (node_index, _) = lv.get_random(rng).unwrap();
            if node_index == lv.get_head_index().unwrap() ||
                node_index == lv.get_tail_index().unwrap(){
                continue;
            }
            shifting_node = node_index;
            break;
        };

        let before_shifting_node = lv.get_prev(shifting_node).unwrap();

        let target_neighbor1: LVNodeIndex;
        loop {
            let (node_index, _) = lv.get_random(rng).unwrap();
            if node_index == shifting_node ||
                node_index == before_shifting_node ||
                node_index == lv.get_tail_index().unwrap() {
                continue;
            }
            target_neighbor1 = node_index;
            break;
        };

        let target_neighbor2: LVNodeIndex = lv.get_next(target_neighbor1).unwrap();

        Some(ShiftInRoute{
            is_truck1,
            day: day_enum,
            time_of_day,
            shifting_node,
            target_neighbor1,
            target_neighbor2,
        })
    }

    pub fn time_difference(&self, truck1: &Week, truck2: &Week) -> Option<Time>{
        let truck = if self.is_truck1 {truck1} else {truck2};
        let route = truck.get(self.day).get(self.time_of_day);
        let lv = &route.linked_vector;

        let orders = get_orders();

        let before_shift = orders[*lv.get_prev_value(self.shifting_node).unwrap()].matrix_id.into();
        let after_shift = orders[*lv.get_next_value(self.shifting_node).unwrap()].matrix_id.into();

        let t1 = orders[*lv.get_value_unsafe(self.target_neighbor1)].matrix_id.into();
        let shift = orders[*lv.get_value_unsafe(self.shifting_node)].matrix_id.into();
        let t2 = orders[*lv.get_value_unsafe(self.target_neighbor2)].matrix_id.into();

        // add the difference between the shifting_node and the two nodes where it fill be put between
        let mut time_difference = time_between_three_nodes(t1, shift, t2);
        // remove the time between these two nodes
        time_difference -= time_between_two_nodes(t1, t2);


        // add the time between the two neighbors of the node that will be shifted
        time_difference += time_between_two_nodes(before_shift, after_shift);
        // remove the time between the node that will be shifted and it's current neighbors
        time_difference -= time_between_three_nodes(before_shift, shift, after_shift);

        // normally we would calculate the emptying time here,
        // but the shifted node will end up in this route again,
        // thus we will end up with the same emptying time for this route.

        Some(time_difference)
    }
}
impl NeighborMove for ShiftInRoute{
    fn evaluate(&self, truck1: &Week, truck2: &Week, _: &OrderFlags) -> Option<CostChange> {
        Some(self.time_difference(truck1, truck2)?)
    }

    fn apply(&self, truck1: &mut Week, truck2: &mut Week, _: &mut OrderFlags) {
        // calculate the change in time after this operation
        let time_difference = self.time_difference(truck1, truck2).unwrap();

        let truck = if self.is_truck1 {truck1} else {truck2};
        let route = truck.get_mut(self.day).get_mut(self.time_of_day);

        route.time += time_difference;

        // move the shifting_node in the lv
        let lv = &mut route.linked_vector;
        let shifting_value = *lv.get_value(self.shifting_node).unwrap();
        lv.remove(self.shifting_node);
        lv.insert_after(self.target_neighbor1, shifting_value);
        // don't need to compact, because the lv has the same length as before the operations.

        route.check_correctness_time();
    }
}
