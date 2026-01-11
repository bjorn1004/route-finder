use rand::Rng;
use crate::datastructures::linked_vectors::{LVNodeIndex, LinkedVector};
use crate::simulated_annealing::day::TimeOfDay;
use crate::simulated_annealing::route::OrderIndex;
use crate::simulated_annealing::simulated_annealing::TruckEnum;
use crate::simulated_annealing::solution::Solution;
use crate::simulated_annealing::week::DayEnum;

pub struct ShiftInDay {
    shift: TruckDayTimeNode,
    target: TruckDayTimeNode,
}


struct TruckDayTimeNode {
    truck: TruckEnum,
    day: DayEnum,
    time_of_day: TimeOfDay,
    node_index: LVNodeIndex,
    order: OrderIndex,
}

impl ShiftInDay {
    pub fn new<R: Rng + ?Sized>(
        solution: &Solution,
        rng: &mut R,
    ) -> Option<Self> {
        let shift = Self::get_shift(solution, rng)?;
        let target = Self::get_target(solution, rng, &shift)?;
        Some(Self { shift, target })
    }

    fn get_shift<R: Rng + ?Sized>(
        solution: &Solution,
        rng: &mut R,
    ) -> Option<TruckDayTimeNode> {
        let truck:TruckEnum = rng.random();
        let day:DayEnum = rng.random();
        let time_of_day:TimeOfDay = rng.random();
        let route = (if truck == TruckEnum::Truck1 {&solution.truck1} else {&solution.truck2}).get(day).get(time_of_day);
        let (node_index, order) = route.linked_vector.get_random(rng).unwrap();
        if node_index == route.linked_vector.get_tail_index().unwrap() ||
            node_index == route.linked_vector.get_head_index().unwrap(){
            None
        } else {
            Some(TruckDayTimeNode{
                truck,
                day,
                time_of_day,
                node_index,
                order: *order,
            })
        }
    }

    fn get_target<R: Rng + ?Sized>(
        solution: &Solution,
        rng: &mut R,
        shift: &TruckDayTimeNode
    ) -> Option<TruckDayTimeNode> {
        todo!()
    }
}