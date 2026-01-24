use std::collections::HashSet;
use rand::Rng;
use crate::datastructures::linked_vectors::{LinkedVector, LVNodeIndex};
use crate::{get_orders};
use crate::resource::{Company, Frequency};
use crate::simulated_annealing::day::TimeOfDay;
use crate::simulated_annealing::neighbor_move::evaluation_helper::{calculate_capacity_overflow, calculate_time_overflow};
use crate::simulated_annealing::route::{OrderIndex};
use crate::simulated_annealing::neighbor_move::evaluation::Evaluation;
use crate::simulated_annealing::neighbor_move::neighbor_move_trait::{NeighborMove, ScoreChange};
use crate::simulated_annealing::order_location::ExactLocation;
use crate::simulated_annealing::simulated_annealing::TruckEnum;
use crate::simulated_annealing::simulated_annealing::TruckEnum::Truck1;
use crate::simulated_annealing::solution::Solution;
use crate::simulated_annealing::week::{DayEnum};
use crate::simulated_annealing::week::DayEnum::{Friday, Monday, Thursday, Tuesday, Wednesday};

/// This will add an order to a random route where it is allowed to add it to.
/// If you try to add an order, that doesn't have any allowed routes, it panics
pub struct AddMultipleNewOrders {
    where_to_add_orders: Vec<AddOrderInfo>,
    order_index: OrderIndex
}

struct AddOrderInfo {
    truck_enum: TruckEnum,
    day: DayEnum,
    time_of_day: TimeOfDay,
    insert_after_index: LVNodeIndex,
}
/// This cannot coexist with the normal Add operation.
impl AddMultipleNewOrders {
    pub fn new<R: Rng+?Sized>(solution: &Solution, rng: &mut R, order_index: OrderIndex) ->  Option<Self>{
        let order = &get_orders()[order_index];
        let mut orders_to_add: Vec<AddOrderInfo> = Vec::new();


        let days_to_add:Vec<DayEnum> = match order.frequency {
            Frequency::None => unreachable!(),
            Frequency::Once => vec![rng.random()],
            Frequency::Twice => {match rng.random_range(0..2) {
                0 => vec![Monday, Thursday],
                1 => vec![Tuesday, Friday],
                _ => unreachable!()
            }}
            Frequency::Thrice => vec![Monday, Wednesday, Friday],
            Frequency::FourTimes => {
                let mut all_days = vec![Monday, Tuesday, Wednesday, Thursday, Friday];
                let mut days_to_add:Vec<DayEnum> = Vec::new();
                for _ in 0..4 {
                    let random_index = rng.random_range(0..all_days.len());
                    days_to_add.push(all_days[random_index]);
                    all_days.remove(random_index);
                }
                let mut set:HashSet<DayEnum> = HashSet::new();
                for i in &days_to_add{
                    set.insert(*i);
                }

                assert_eq!(set.len(), days_to_add.len());
                days_to_add
            }
        };
        for day in days_to_add {

            if let Some(order_to_add) = Self::get_random_allowed_order(solution, rng, day, order) {
                orders_to_add.push(order_to_add);
            } else {
                return None
            }
        }
        Some(AddMultipleNewOrders{
            where_to_add_orders: orders_to_add,
            order_index
        })
    }

    fn get_random_allowed_order<R: Rng + ?Sized>(solution: &Solution, rng: &mut R, day_enum: DayEnum, order: &Company) -> Option<AddOrderInfo>{

        let truck_enum: TruckEnum = rng.random();
        let truck = if truck_enum == TruckEnum::Truck1 {&solution.truck1} else {&solution.truck2};

        let day = truck.get(day_enum);
        let (route, time_of_day_enum) = day.get_random(rng);

        let lv = &route.linked_vector;
        while let Some((index, _)) = lv.get_random(rng) {
            if lv.get_tail_index() == Some(index) {
                // we don't want to add behind the tail.
                // we could try to instead insert in front of the tail but don't want to try that rn.
                // instead we try to randomly find a new value which hopefully isn't the tail
                continue
            }


            return Some(AddOrderInfo {
                truck_enum,
                day: day_enum,
                time_of_day: time_of_day_enum,
                insert_after_index: index,
            })
        }
        panic!("how did we get here?")

    }
}


impl NeighborMove for AddMultipleNewOrders {
    fn evaluate(&self, solution: &Solution) -> Evaluation {
        let orders = get_orders();
        let order = &orders[self.order_index];
        let evaluation:Evaluation = self.where_to_add_orders
            .iter()
            .map(|order_info| {
                let day = (if order_info.truck_enum == TruckEnum::Truck1 { &solution.truck1 } else {&solution.truck2})
                    .get(order_info.day);
                let route = day.get(order_info.time_of_day);
                // calculate the time it takes to do add this order
                let time_diff =
                    route.calculate_add_order(order_info.insert_after_index, self.order_index);

                let time_overflow_delta =
                    calculate_time_overflow(time_diff, day.get_total_time());
                let capacity_overflow_delta =
                    calculate_capacity_overflow(order.total_container_volume as i32, route.capacity as i32);

                Evaluation{
                    cost: time_diff,
                    time_overflow_delta,
                    capacity_overflow_delta
                }
            })
            .sum();


        Evaluation{
            cost: evaluation.cost - get_orders()[self.order_index].penalty,
            ..evaluation
        }
    }

    fn apply(&self, solution: &mut Solution) -> ScoreChange {

        // let mut truck1 = solution.truck1.get_all_as_mut();
        // let mut truck2 = solution.truck2.get_all_as_mut();
        let mut total_score_change = 0;

        let mut indices = vec![];
        for order_info in &self.where_to_add_orders {
            let truck = if order_info.truck_enum == Truck1 {&mut solution.truck1} else {&mut solution.truck2};
            let day = truck.get_mut(order_info.day);
            let route = day.get_mut(order_info.time_of_day);


            total_score_change += route.apply_add_order(
                order_info.insert_after_index,
                self.order_index
            );

            indices.push(route.linked_vector.len() - 1);
        }

        let frequency = match indices.len(){
            1 => Frequency::Once,
            2 => Frequency::Twice,
            3 => Frequency::Thrice,
            4 => Frequency::FourTimes,
            _ => unreachable!()
        };

        solution.order_location.add_multiple(self.order_index,
            self.where_to_add_orders
                .iter()
                .zip(indices)
                .map(|(info, node_i)|
                    ExactLocation{
                        truck_enum: info.truck_enum,
                        day_enum: info.day,
                        time_of_day: info.time_of_day,
                        node_index: node_i,
                    }).collect(), frequency);

        // We can always subtract the penalty, because this operation will add the order on as many days as needed to mee the frequency requirement.
        total_score_change - get_orders()[self.order_index].penalty
    }
}
