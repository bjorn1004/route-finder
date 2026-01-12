use petgraph::matrix_graph::Nullable;
use rand::Rng;
use DayEnum::{Monday, Tuesday, Wednesday};
use crate::datastructures::linked_vectors::{LVNodeIndex, LinkedVector};
use crate::get_orders;
use crate::simulated_annealing::day::TimeOfDay;
use crate::simulated_annealing::neighbor_move::evaluation_helper::{calculate_capacity_overflow, calculate_time_overflow};
use crate::simulated_annealing::neighbor_move::neighbor_move_trait::{Evaluation, NeighborMove, ScoreChange};
use crate::simulated_annealing::order_day_flags::OrderFlags;
use crate::simulated_annealing::route::{OrderIndex, Route};
use crate::simulated_annealing::simulated_annealing::TruckEnum;
use crate::simulated_annealing::solution::Solution;
use crate::simulated_annealing::week::{DayEnum, Week};
use crate::simulated_annealing::week::DayEnum::{Friday, Thursday};

pub struct ShiftBetweenDays {
    shifts: [Option<TruckDayTimeNode>; 2],
    targets: [Option<TruckDayTimeNode>; 2],
    order: OrderIndex,
}
struct TruckDayTimeNode{
    truck: TruckEnum,
    day: DayEnum,
    time_of_day: TimeOfDay,
    node_index: LVNodeIndex,
}

impl ShiftBetweenDays {
    pub fn new<R: Rng + ?Sized>(
        solution: &Solution,
        rng: &mut R,
    ) -> Option<Self> {
        let (first_thingy, shift_order_index) = Self::find_first_random_node(solution, rng)?;

        let order = &get_orders()[shift_order_index];
        // if the order has frequency 3 or 5, we can't shift to another place
        let frequency = order.frequency as u8;
        if frequency == 3 ||
            frequency == 5 {
            return None;
        }



        let mut second_shift: Option<TruckDayTimeNode> = None;

        let targets = match frequency {
            1 => {
                // this is my attempt at getting a random number between 0 and 4 that is different from the shift day.
                // random day between 0 and 3.
                let random_num = rng.random_range(0..4);
                let target_day = if random_num == first_thingy.day as u8 {
                    // if we got the same day as the shift day, we use day 4 (0 based indexing)
                    Friday
                } else {
                    // just use the random number we found.
                    match random_num {
                        0 => Monday,
                        1 => Tuesday,
                        2 => Wednesday,
                        3 => Thursday,
                        _ => unreachable!(),
                    }
                };
                [Some(Self::find_random_target(solution, rng, target_day)?), None]},
            2 => {
                let other_shift_day = match first_thingy.day {
                    Monday => Thursday,
                    Tuesday => Friday,
                    Wednesday => unreachable!(),
                    Thursday => Monday,
                    Friday => Tuesday,
                };
                let other_shift_node = Self::find_other_day(solution, other_shift_day, shift_order_index);
                second_shift = Some(other_shift_node);

                let target_days = match first_thingy.day {
                    Monday => [Tuesday, Friday],
                    Tuesday => [Monday, Thursday],
                    Wednesday => unreachable!(),
                    Thursday => [Tuesday, Friday],
                    Friday => [Monday, Thursday],
                };

                [Some(Self::find_random_target(solution, rng, target_days[0])?),
                    Some(Self::find_random_target(solution, rng, target_days[1])?)]
            },
            3 => return None,
            4 => {
                return None;
                [Some(Self::find_random_target(solution, rng, Monday)?), None]
            },
            5 => return None,
            _ => unreachable!()
        };
        Some(Self {
            shifts: [Some(first_thingy), second_shift],
            targets,
            order: shift_order_index,
        })
    }
    fn find_first_random_node<R: Rng + ?Sized>(solution: &Solution, rng: &mut R) -> Option<(TruckDayTimeNode, OrderIndex)>{
        let truck:TruckEnum = rng.random();
        let day: DayEnum = rng.random();
        let time_of_day: TimeOfDay = rng.random();

        let route = (if truck == TruckEnum::Truck1 {&solution.truck1} else {&solution.truck2}).get(day).get(time_of_day);

        let (node_index, order) = route.linked_vector.get_random(rng).unwrap();

        if node_index == route.linked_vector.get_head_index().unwrap() ||
            node_index == route.linked_vector.get_tail_index().unwrap(){
            return None;
        }

        Some((TruckDayTimeNode{
            truck,
            day,
            time_of_day,
            node_index,
        }, *order))
    }
    // COPIED SHIT FROM REMOVE MULTIPLE AT ONCE BECAUSE I LAZY
    // WILL BE CLEANED UP ONCE WE HAVE A NEW STRUCT TO FIND OTHER OCCURENCES OF AN ORDER IN O(1) TIME
    fn find_other_day(solution: &Solution, day_enum: DayEnum, order_index: OrderIndex) -> TruckDayTimeNode {
        if let Some((time_of_day, node_index)) = Self::find_other_day_in_truck(&solution.truck1, day_enum, order_index) {
            return TruckDayTimeNode{
                truck: TruckEnum::Truck1,
                day: day_enum,
                time_of_day,
                node_index,
            }
        } else if let Some((time_of_day, node_index)) = Self::find_other_day_in_truck(&solution.truck2, day_enum, order_index) {
            return TruckDayTimeNode{
                truck: TruckEnum::Truck2,
                day: day_enum,
                time_of_day,
                node_index,
            }
        }

        panic!("couldn't find the order to remove. Something probably went wrong with the orderflags")
    }

    fn find_other_day_in_truck(truck: &Week, day_enum: DayEnum, order_index: OrderIndex) -> Option<(TimeOfDay, LVNodeIndex)>{
        let day = truck.get(day_enum);
        if let Some(node_index) = Self::find_other_day_in_route(day.get(TimeOfDay::Morning), order_index) {
            Some((TimeOfDay::Morning, node_index))
        } else if let Some(node_index) = Self::find_other_day_in_route(day.get(TimeOfDay::Afternoon), order_index) {
            Some((TimeOfDay::Afternoon, node_index))
        } else {
            None
        }
    }

    fn find_other_day_in_route(route: &Route, order_index: OrderIndex) -> Option<LVNodeIndex> {
        for (node_index, _order_index) in route.linked_vector.iter(){
            if *_order_index == order_index {
                return Some(node_index);
            }
        }
        None
    }
    /// gets a random target_node on the given day. This could be any node in a route besides the tail.
    fn find_random_target<R: Rng + ?Sized>(solution: &Solution, rng: &mut R, day_enum: DayEnum) -> Option<TruckDayTimeNode>{
        let truck:TruckEnum = rng.random();
        let day = solution.get_truck(truck).get(day_enum);
        let time_of_day = rng.random();
        let route = day.get(time_of_day);

        let (node_index, _order_index) = route.linked_vector.get_random(rng)?;

        if node_index == route.linked_vector.get_tail_index()?{
            return None;
        }

        Some(TruckDayTimeNode{
            truck,
            day: day_enum,
            time_of_day,
            node_index,
        })
    }
    /// I call this function once most of the time with i=0. This gets the first element in the shift and target array.
    /// If there is a frequency 2 order, this function is also called with i=1.
    fn evaluation_helper(&self, solution: &Solution, i: usize) -> Evaluation {
        let shift_info = self.shifts[i].as_ref().unwrap();
        let shift_day = solution.get_truck(shift_info.truck).get(shift_info.day);
        let shift_route = shift_day.get(shift_info.time_of_day);

        let shift_diff = shift_route.calculate_remove_node(shift_info.node_index);

        let target_info = self.targets[i].as_ref().unwrap();
        let target_day = solution.get_truck(target_info.truck).get(target_info.day);
        let target_route = target_day.get(target_info.time_of_day);

        let target_diff = target_route.calculate_add_order(target_info.node_index, self.order);

        let (shift_t_overflow, shift_t_lessened) = calculate_time_overflow(shift_diff, shift_day.get_total_time());
        let (target_t_overflow, target_t_lesssened) = calculate_time_overflow(target_diff, target_day.get_total_time());

        let orders = get_orders();
        let order = &orders[self.order];
        let (shift_c_overflow, shift_c_lessened) = calculate_capacity_overflow(-(order.total_container_volume as i32), shift_route.capacity as i32);
        let (target_c_overflow, target_c_lessened) = calculate_capacity_overflow(order.total_container_volume as i32, target_route.capacity as i32);

        let a = Evaluation {
            cost: shift_diff + target_diff,
            time_overflow: shift_t_overflow + target_t_overflow,
            time_overflow_lessened: shift_t_lessened + target_t_lesssened,
            capacity_overflow: shift_c_overflow + target_c_overflow,
            capacity_overflow_lessened: shift_c_lessened + target_c_lessened,
        };
        a.validate()
    }
}

impl NeighborMove for ShiftBetweenDays {
    fn evaluate(&self, solution: &Solution) -> Evaluation {
        let first_shift = self.evaluation_helper(solution, 0);
        // if we are doing a shift for frequency 2, also calaculate the second stored shift
        if self.shifts[1].is_some(){
            return self.evaluation_helper(solution, 1) + first_shift;
        }
        first_shift
    }

    fn apply(&self, solution: &mut Solution) -> ScoreChange {
        // first we clear the order_flags
        solution.order_flags.clear(self.order);

        // get all the data and apply the remove and adds
        let shift_info = self.shifts[0].as_ref().unwrap();
        let shift_route = solution.get_truck_mut(shift_info.truck)
            .get_mut(shift_info.day)
            .get_mut(shift_info.time_of_day);
        let shift_diff = shift_route.apply_remove_node(shift_info.node_index);

        let target_info = self.targets[0].as_ref().unwrap();
        let target_route = solution.get_truck_mut(target_info.truck)
            .get_mut(target_info.day)
            .get_mut(target_info.time_of_day);
        let target_diff = target_route.apply_add_order(target_info.node_index, self.order);

        // update the order flags
        solution.order_flags.add_order(self.order, target_info.day);

        // ugly code that maybe works
        if self.shifts[1].is_some(){
            let shift_info = self.shifts[1].as_ref().unwrap();
            let shift_route = solution.get_truck_mut(shift_info.truck)
                .get_mut(shift_info.day)
                .get_mut(shift_info.time_of_day);
            let shift_diff2 = shift_route.apply_remove_node(shift_info.node_index);

            let target_info = self.targets[1].as_ref().unwrap();
            let target_route = solution.get_truck_mut(target_info.truck)
                .get_mut(target_info.day)
                .get_mut(target_info.time_of_day);
            let target_diff2 = target_route.apply_add_order(target_info.node_index, self.order);

            // update the order flags
            solution.order_flags.add_order(self.order, target_info.day);

            return shift_diff + target_diff + shift_diff2 + target_diff2;
        }


        shift_diff + target_diff
    }
}