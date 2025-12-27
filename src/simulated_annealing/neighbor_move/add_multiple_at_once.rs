use rand::Rng;
use crate::datastructures::linked_vectors::{LinkedVector, LVNodeIndex};
use crate::{get_orders};
use crate::resource::{Company, Time, HALF_HOUR};
use crate::simulated_annealing::day::TimeOfDay;
use crate::simulated_annealing::order_day_flags::OrderFlags;
use crate::simulated_annealing::route::{OrderIndex, Route};
use crate::simulated_annealing::neighbor_move::neighbor_move_trait::{CostChange, NeighborMove, ScoreChange};
use crate::simulated_annealing::simulated_annealing::TruckEnum;
use crate::simulated_annealing::solution::Solution;
use crate::simulated_annealing::week::{DayEnum};

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
        let mut flags = 0;
        for _ in 0..order.frequency as u8 {
            if let Some(order_to_add) = Self::get_random_allowed_order(solution, rng, flags, order) {
                flags |= OrderFlags::day_to_flags(order_to_add.day);
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

    fn get_random_allowed_order<R: Rng + ?Sized>(solution: &Solution, rng: &mut R, flags: u8, order: &Company) -> Option<AddOrderInfo>{

        let truck_enum: TruckEnum = rng.random();
        let truck = if truck_enum == TruckEnum::Truck1 {&solution.truck1} else {&solution.truck2};

        // check if there is still an allowed day open
        if let Some(day_enum) = OrderFlags::_get_random_allowed_day(flags, order.frequency, rng){
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
        } else {
            panic!("We tried to add an order, but there are no days days left where this order could be added")
        }

    }


    fn calculate_time_difference(&self, solution: &Solution, order_to_add: &AddOrderInfo) -> Time{
        let orders = get_orders();
        let order = &orders[self.order_index];
        let route = (if order_to_add.truck_enum == TruckEnum::Truck1 { &solution.truck1 } else {&solution.truck2}).get(order_to_add.day).get(order_to_add.time_of_day);

        let time = route.calculate_add_order(order_to_add.insert_after_index, self.order_index);
        time + order.emptying_time
    }

    fn apply_on_one_route(&self, route: &mut Route, order_info: &AddOrderInfo) -> Time {

        let score = route.apply_add_order(order_info.insert_after_index, self.order_index);

        if route.linked_vector.len() == 3{
            return score + HALF_HOUR;
        };

        score
    }
}


impl NeighborMove for AddMultipleNewOrders {
    fn evaluate(&self, solution: &Solution) -> CostChange {
        self.where_to_add_orders
            .iter()
            .map(|order_info| self.calculate_time_difference(solution, order_info))
            .sum::<Time>()
            // We can always subtract the penalty, becuase this operation will add the order on as many days as needed to mee the frequency requirement.
            - get_orders()[self.order_index].penalty
    }

    fn apply(&self, solution: &mut Solution) -> ScoreChange {

        let mut truck1 = solution.truck1.get_all_as_mut();
        let mut truck2 = solution.truck2.get_all_as_mut();
        let mut total_score_change = 0;
        for order_info in &self.where_to_add_orders {
            if order_info.truck_enum == TruckEnum::Truck1{
                total_score_change +=  self.apply_on_one_route(truck1[order_info.day as usize * 2 + order_info.time_of_day as usize], order_info);
            } else {
                total_score_change += self.apply_on_one_route(truck2[order_info.day as usize * 2 + order_info.time_of_day as usize], order_info);
            }
            solution.order_flags.add_order(self.order_index, order_info.day);
        }

        // We can always subtract the penalty, becuase this operation will add the order on as many days as needed to mee the frequency requirement.
        total_score_change - get_orders()[self.order_index].penalty
    }
}
