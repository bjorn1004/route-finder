use crate::datastructures::linked_vectors::{LVNodeIndex, LinkedVector};
use crate::resource::Time;
use crate::simulated_annealing::day::TimeOfDay;
use crate::simulated_annealing::neighbor_move::neighbor_move_trait::{CostChange, NeighborMove};
use crate::simulated_annealing::order_day_flags::OrderFlags;
use crate::simulated_annealing::week::{DayEnum};
use rand::Rng;
use crate::simulated_annealing::solution::Solution;

pub struct ShiftInRoute {
    truck1: bool,
    day: DayEnum,
    time_of_day: TimeOfDay,
    shifting_node: LVNodeIndex,
    target_neighbor1: LVNodeIndex,
}

impl ShiftInRoute {
    pub fn new<R: Rng + ?Sized>(solution: &Solution, rng: &mut R) -> Option<Self> {
        let truck1: bool = rng.random();
        let truck = if truck1 { &solution.truck1 } else { &solution.truck2 };

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

        Some(ShiftInRoute {
            truck1,
            day: day_enum,
            time_of_day,
            shifting_node,
            target_neighbor1,
        })
    }

    pub fn time_difference(&self, solution: &Solution) -> Time {

        let truck = if self.truck1 { &solution.truck1 } else { &solution.truck2 };
        let route = truck.get(self.day).get(self.time_of_day);
        let lv = &route.linked_vector;

        route.calculate_remove_node(self.shifting_node) +
            route.calculate_add_order(self.target_neighbor1, *lv.get_value_unsafe(self.shifting_node))
    }
}
impl NeighborMove for ShiftInRoute {
    fn evaluate(&self, solution: &Solution) -> CostChange {
        self.time_difference(&solution)
    }

    fn apply(&self, solution: &mut Solution) -> Time {
        // calculate the change in time after this operation
        let time_difference = self.time_difference(solution);

        let truck = if self.truck1 { &mut solution.truck1 } else { &mut solution.truck2 };
        let route = truck.get_mut(self.day).get_mut(self.time_of_day);

        route.time += time_difference;

        // move the shifting_node in the lv
        let lv = &mut route.linked_vector;
        let shifting_value = *lv.get_value(self.shifting_node).unwrap();
        lv.remove(self.shifting_node);
        lv.insert_after(self.target_neighbor1, shifting_value);
        // don't need to compact, because the lv has the same length as before the operations.

        #[cfg(debug_assertions)]
        route.check_correctness_time();

        time_difference
    }
}
