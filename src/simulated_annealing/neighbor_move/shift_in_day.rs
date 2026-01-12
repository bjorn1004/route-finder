use rand::Rng;
use crate::datastructures::linked_vectors::{LVNodeIndex, LinkedVector};
use crate::get_orders;
use crate::simulated_annealing::day::TimeOfDay;
use crate::simulated_annealing::neighbor_move::evaluation_helper::{calculate_capacity_overflow, calculate_time_overflow};
use crate::simulated_annealing::neighbor_move::neighbor_move_trait::{Evaluation, NeighborMove, ScoreChange};
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

        // if the random node is a tail or head, we can't shift it.
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
        let random_truck = rng.random_range(0..3);
        // random number between 0 and 2.
        // each number is a different route.
        // This is to give all 3 routes an equal chance of being selected.
        let (truck, day, time_of_day) = match random_truck {
            // same truck, other time_of_day
            0 => (shift.truck, shift.day, Self::other_time_of_day(shift.time_of_day)),
            // other truck, same time_of_day
            1 => (Self::other_truck(shift.truck), shift.day, shift.time_of_day),
            // other truck, other time_of_day
            2 => (Self::other_truck(shift.truck), shift.day, Self::other_time_of_day(shift.time_of_day)),
            // range 0..3 means anything besides 0, 1 and 2 does not happen
            _ => unreachable!(),
        };

        let route = (if truck == TruckEnum::Truck1 {&solution.truck1} else {&solution.truck2}).get(day).get(time_of_day);
        let (node_index, order) = route.linked_vector.get_random(rng).unwrap();

        // If the random node is the tail, we can't shift behind it.
        // If the random node is the head, we can shift behind it.
        if node_index == route.linked_vector.get_tail_index().unwrap(){
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
    fn other_truck(truck: TruckEnum) -> TruckEnum {
        match truck {
            TruckEnum::Truck1 => TruckEnum::Truck2,
            TruckEnum::Truck2 => TruckEnum::Truck1,
        }
    }
    fn other_time_of_day(time_of_day: TimeOfDay) -> TimeOfDay {
        match time_of_day {
            TimeOfDay::Morning => TimeOfDay::Afternoon,
            TimeOfDay::Afternoon => TimeOfDay::Morning,
        }
    }

}

impl NeighborMove for ShiftInDay {
    fn evaluate(&self, solution: &Solution) -> Evaluation {
        let shift_day = (if self.shift.truck == TruckEnum::Truck1 {&solution.truck1} else {&solution.truck2}).get(self.shift.day);
        let shift_route = shift_day.get(self.shift.time_of_day);

        let target_day = (if self.target.truck == TruckEnum::Truck1 {&solution.truck1} else {&solution.truck2}).get(self.target.day);
        let target_route = target_day.get(self.target.time_of_day);

        let shift_diff = shift_route.calculate_remove_node(self.shift.node_index);
        let target_diff = target_route.calculate_add_order(self.target.node_index, self.shift.order);

        let (shift_t_overflow, shift_t_lessened) = calculate_time_overflow(shift_diff, shift_day.get_total_time());
        let (target_t_overflow, target_t_lesssened) = calculate_time_overflow(target_diff, target_day.get_total_time());


        let orders = get_orders();
        let order = &orders[self.shift.order];
        let (shift_c_overflow, shift_c_lessened) = calculate_capacity_overflow(-(order.total_container_volume as i32), shift_route.capacity as i32);
        let (target_c_overflow, target_c_lessened) = calculate_capacity_overflow(order.total_container_volume as i32, target_route.capacity as i32);

        Evaluation {
            cost: shift_diff + target_diff,
            time_overflow: shift_t_overflow + target_t_overflow,
            time_overflow_lessened: shift_t_lessened + target_t_lesssened,
            capacity_overflow: shift_c_overflow + target_c_overflow,
            capacity_overflow_lessened: shift_c_lessened + target_c_lessened,
        }
    }

    fn apply(&self, solution: &mut Solution) -> ScoreChange {
        let shift_day = (if self.shift.truck == TruckEnum::Truck1 {&mut solution.truck1} else {&mut solution.truck2}).get_mut(self.shift.day);
        let shift_route = shift_day.get_mut(self.shift.time_of_day);

        let shift_diff = shift_route.apply_remove_node(self.shift.node_index);

        let target_route = (if self.target.truck == TruckEnum::Truck1 {&mut solution.truck1} else {&mut solution.truck2}).get_mut(self.target.day).get_mut(self.target.time_of_day);
        
        let target_diff = target_route.apply_add_order(self.target.node_index, self.shift.order);
        shift_diff + target_diff
    }
}