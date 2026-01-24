use rand::Rng;
use DayEnum::{Monday, Tuesday, Wednesday};
use crate::datastructures::linked_vectors::{LinkedVector};
use crate::get_orders;
use crate::resource::Frequency;
use crate::simulated_annealing::day::TimeOfDay;
use crate::simulated_annealing::neighbor_move::evaluation_helper::{calculate_capacity_overflow, calculate_time_overflow};
use crate::simulated_annealing::neighbor_move::evaluation::Evaluation;
use crate::simulated_annealing::neighbor_move::neighbor_move_trait::{NeighborMove, ScoreChange};
use crate::simulated_annealing::order_location::{ExactLocation, OrderInfo};
use crate::simulated_annealing::order_location::OrderInfo::Frice;
use crate::simulated_annealing::route::{OrderIndex};
use crate::simulated_annealing::simulated_annealing::TruckEnum;
use crate::simulated_annealing::solution::Solution;
use crate::simulated_annealing::week::{DayEnum};
use crate::simulated_annealing::week::DayEnum::{Friday, Thursday};

pub struct ShiftBetweenDays {
    shifts: [Option<ExactLocation>; 2],
    targets: [Option<ExactLocation>; 2],
    order: OrderIndex,
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



        let mut second_shift: Option<ExactLocation> = None;

        let targets = match frequency {
            1 => {
                // this is my attempt at getting a random number between 0 and 4 that is different from the shift day.
                // random day between 0 and 3.
                let random_num = rng.random_range(0..4);
                let target_day = if random_num == first_thingy.day_enum as u8 {
                    // if we got the same day as the shift day, we use day 4 (0-based indexing)
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
                let order_info = solution.order_location.get_order_location(shift_order_index);

                let shift_info = match order_info {
                    OrderInfo::Twice(a) => a, 
                    _ => unreachable!()
                };


                second_shift = if shift_info[0].day_enum == first_thingy.day_enum {
                    #[cfg(debug_assertions)]
                    assert_eq!(shift_info[0], first_thingy);
                    Some(shift_info[1])
                } else {
                    #[cfg(debug_assertions)]
                    assert_eq!(shift_info[1], first_thingy);
                    Some(shift_info[0])
                };

                let target_days = match first_thingy.day_enum {
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
                if let Frice(info) = solution.order_location.get_order_location(shift_order_index){

                    let mut flags:u8 = 0b0_0000;
                    for day in info.iter().map(|exact_location | exact_location.day_enum){
                        flags |= match day {
                            Monday    => 0b1_0000,
                            Tuesday   => 0b0_1000,
                            Wednesday => 0b0_0100,
                            Thursday  => 0b0_0010,
                            Friday    => 0b0_0001,
                        }
                    }

                    // Flip all bits, then mask to keep only the 5-day bits
                    let available_days_mask = (!flags) & 0b11111;

                    let other_day = match available_days_mask {
                        0b1_0000 => Monday    ,
                        0b0_1000 => Tuesday   ,
                        0b0_0100 => Wednesday ,
                        0b0_0010 => Thursday  ,
                        0b0_0001 => Friday    ,
                        _ => unreachable!()
                    };

                    [Some(Self::find_random_target(solution, rng, other_day)?), None]
                } else {
                    unreachable!()
                }
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
    fn find_first_random_node<R: Rng + ?Sized>(solution: &Solution, rng: &mut R) -> Option<(ExactLocation, OrderIndex)>{
        let truck_enum:TruckEnum = rng.random();
        let day_enum: DayEnum = rng.random();
        let time_of_day: TimeOfDay = rng.random();

        let route = (if truck_enum == TruckEnum::Truck1 {&solution.truck1} else {&solution.truck2}).get(day_enum).get(time_of_day);

        let (node_index, order) = route.linked_vector.get_random(rng).unwrap();

        if node_index == route.linked_vector.get_head_index().unwrap() ||
            node_index == route.linked_vector.get_tail_index().unwrap(){
            return None;
        }

        Some((ExactLocation{
            truck_enum,
            day_enum,
            time_of_day,
            node_index,
        }, *order))
    }
    /// Gets a random target_node on the given day. This could be any node in a route besides the tail.
    fn find_random_target<R: Rng + ?Sized>(solution: &Solution, rng: &mut R, day_enum: DayEnum) -> Option<ExactLocation>{
        let truck_enum:TruckEnum = rng.random();
        let day = solution.get_truck(truck_enum).get(day_enum);
        let time_of_day = rng.random();
        let route = day.get(time_of_day);


        loop {
            let (node_index, _order_index) = route.linked_vector.get_random(rng)?;
            if node_index == route.linked_vector.get_tail_index()?{
                continue;
            }

            return Some(ExactLocation{
                truck_enum,
                day_enum,
                time_of_day,
                node_index,
            });
        }
    }
    /// I call this function once most of the time with i=0. This gets the first element in the shift and target array.
    /// If there is a frequency 2 order, this function is also called with i=1.
    fn evaluation_helper(&self, solution: &Solution, i: usize) -> Evaluation {
        let shift_info = self.shifts[i].as_ref().unwrap();
        let shift_day = solution.get_truck(shift_info.truck_enum).get(shift_info.day_enum);
        let shift_route = shift_day.get(shift_info.time_of_day);

        let shift_diff = shift_route.calculate_remove_node(shift_info.node_index);

        let target_info = self.targets[i].as_ref().unwrap();
        let target_day = solution.get_truck(target_info.truck_enum).get(target_info.day_enum);
        let target_route = target_day.get(target_info.time_of_day);

        let target_diff = target_route.calculate_add_order(target_info.node_index, self.order);

        let shift_t_delta = calculate_time_overflow(shift_diff, shift_day.get_total_time());
        let target_t_delta = calculate_time_overflow(target_diff, target_day.get_total_time());

        let orders = get_orders();
        let order = &orders[self.order];
        let shift_c_delta = calculate_capacity_overflow(-(order.total_container_volume as i32), shift_route.capacity as i32);
        let target_c_delta = calculate_capacity_overflow(order.total_container_volume as i32, target_route.capacity as i32);

        Evaluation {
            cost: shift_diff + target_diff,
            time_overflow_delta: shift_t_delta + target_t_delta,
            capacity_overflow_delta: shift_c_delta + target_c_delta,
        }
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
        // first we clear the order_flags,
        // get all the data and apply the remove and adds
        let shift_info = self.shifts[0].as_ref().unwrap();
        let shift_route = solution.get_truck_mut(shift_info.truck_enum)
            .get_mut(shift_info.day_enum)
            .get_mut(shift_info.time_of_day);

        let (shift_diff, compact_changes) = shift_route.apply_remove_node(shift_info.node_index);
        solution.order_location.update_shift_in_route(compact_changes[0], shift_info.day_enum);

        let target_info = self.targets[0].as_ref().unwrap();
        let target_route = solution.get_truck_mut(target_info.truck_enum)
            .get_mut(target_info.day_enum)
            .get_mut(target_info.time_of_day);
        let target_diff = target_route.apply_add_order(target_info.node_index, self.order);

        let new_exact_location: ExactLocation = ExactLocation{
            truck_enum: target_info.truck_enum,
            day_enum: target_info.day_enum,
            time_of_day: target_info.time_of_day,
            node_index: target_route.linked_vector.len() - 1,
        };
        // ugly code that maybe works
        if self.shifts[1].is_some(){
            let shift_info = self.shifts[1].as_ref().unwrap();
            let shift_route = solution.get_truck_mut(shift_info.truck_enum)
                .get_mut(shift_info.day_enum)
                .get_mut(shift_info.time_of_day);
            let (shift_diff2, compact_changes) = shift_route.apply_remove_node(shift_info.node_index);
            solution.order_location.update_shift_in_route(compact_changes[0], shift_info.day_enum);

            let target_info = self.targets[1].as_ref().unwrap();
            let target_route = solution.get_truck_mut(target_info.truck_enum)
                .get_mut(target_info.day_enum)
                .get_mut(target_info.time_of_day);

            let second_exact_location = ExactLocation {
                truck_enum: target_info.truck_enum,
                day_enum: target_info.day_enum,
                time_of_day: target_info.time_of_day,
                node_index: target_route.linked_vector.len(),
            };

            let target_diff2 = target_route.apply_add_order(target_info.node_index, self.order);

            solution.order_location.remove_order(self.order);
            solution.order_location.add_multiple(self.order, vec![
                new_exact_location,
                second_exact_location
            ], Frequency::Twice);

            return shift_diff + target_diff + shift_diff2 + target_diff2;
        }

        let orders = get_orders();
        match orders[self.order].frequency {
            Frequency::Once => {} // don't need to change anything
            Frequency::FourTimes => {
                if let Frice(mut arr) = solution.order_location.get_order_location(self.order){
                    if let Some(i) = arr.iter().position(|o| o.day_enum == shift_info.day_enum) {
                        arr[i] = new_exact_location;
                        solution.order_location.remove_order(self.order);
                        solution.order_location.add_multiple(self.order, arr.to_vec(), get_orders()[self.order].frequency);
                    } else {
                        unreachable!()
                    }
                } else {
                    panic!();
                }

            }
            _ => unreachable!()
        }


        shift_diff + target_diff
    }
}