use rand::Rng;
use crate::datastructures::linked_vectors::{LinkedVector, NodeIndex};
use crate::simulated_annealing::day::TimeOfDay;
use crate::simulated_annealing::neighbor_move::neighbor_move_trait::{Evaluation, NeighborMove};
use crate::simulated_annealing::order_day_flags::OrderFlags;
use crate::simulated_annealing::route::OrderIndex;
use crate::simulated_annealing::week::{DayEnum, Week};

pub struct ShiftInRoute{
    is_truck1: bool,
    day: DayEnum,
    time_of_day: TimeOfDay,
    shifting_node: NodeIndex,
    target_neighbor1: NodeIndex,
    target_neighbor2: NodeIndex,
}

impl ShiftInRoute{
    pub fn new<R: Rng+?Sized>(truck1: &Week, truck2: &Week, rng: &mut R) ->  Self {
        let is_truck1:bool = rng.random();
        let truck = if is_truck1 {truck1} else {truck2};

        let day_enum:DayEnum = rng.random();
        let day = truck.get(day_enum);

        let time_of_day:TimeOfDay = rng.random();
        let route = day.get(time_of_day);

        let lv = &route.linked_vector;
        let mut shifting_node:NodeIndex;
        loop {
            let (node_index, value) = lv.get_random(rng).unwrap();
            if node_index == lv.get_head_index().unwrap() ||
                node_index == lv.get_tail_index().unwrap(){
                continue;
            }
            shifting_node = node_index;
            break;
        };

        let before_shifting_node = lv.get_prev(shifting_node).unwrap();

        let mut target_neighbor1: NodeIndex;
        loop {
            let (node_index, value) = lv.get_random(rng).unwrap();
            if node_index == shifting_node ||
                node_index == before_shifting_node ||
                node_index == lv.get_tail_index().unwrap() ||
                node_index == shifting_node{
                continue;
            }
            target_neighbor1 = node_index;
            break;
        };

        let target_neighbor2: NodeIndex = lv.get_next(target_neighbor1).unwrap();

        ShiftInRoute{
            is_truck1,
            day: day_enum,
            time_of_day,
            shifting_node,
            target_neighbor1,
            target_neighbor2,
        }
    }
}
impl NeighborMove for ShiftInRoute{
    fn evaluate(&self, truck1: &Week, truck2: &Week, order_flags: &OrderFlags) -> Option<Evaluation> {
        Some(Evaluation{
            cost: 0.0,
        })
    }

    fn apply(&self, truck1: &mut Week, truck2: &mut Week, order_flags: &mut OrderFlags) {
        let truck = if self.is_truck1 {truck1} else {truck2};
        let route = truck.get_mut(self.day).get_mut(self.time_of_day);
        let lv = &mut route.linked_vector;

        let shifting_value = lv.get_value(self.shifting_node).unwrap().clone();
        lv.remove(self.shifting_node);
        lv.insert_after(self.target_neighbor1, shifting_value);
    }
}
