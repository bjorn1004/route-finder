use rand::Rng;
use DayEnum::{Monday, Tuesday, Wednesday};
use crate::datastructures::linked_vectors::{LVNodeIndex, LinkedVector};
use crate::get_orders;
use crate::simulated_annealing::day::TimeOfDay;
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
            1 => [Some(Self::find_random_target(solution, rng, first_thingy.day)?), None],
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
                // this is my attempt at getting a random number between 1 and 5 that is different from the shift day.
                // random day between 1 and 4.
                let random_num = rng.random_range(1..5);
                let target_day = if random_num == first_thingy.day as u8 {
                    // if we got the same day as the shift day, we use day 5
                    Friday
                } else {
                    // just use the random number we found.
                    match random_num {
                        1 => Monday,
                        2 => Tuesday,
                        3 => Wednesday,
                        4 => Thursday,
                        _ => unreachable!(),
                    }
                };

                [Some(Self::find_random_target(solution, rng, target_day)?), None]
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
    fn find_random_target<R: Rng + ?Sized>(solution: &Solution, rng: &mut R, day_enum: DayEnum) -> Option<TruckDayTimeNode>{
        todo!()
    }
}

impl NeighborMove for ShiftBetweenDays {
    fn evaluate(&self, solution: &Solution) -> Evaluation {
        todo!()
    }

    fn apply(&self, solution: &mut Solution) -> ScoreChange {
        todo!()
    }
}