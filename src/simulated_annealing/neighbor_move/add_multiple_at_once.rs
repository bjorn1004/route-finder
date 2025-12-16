use rand::Rng;
use crate::datastructures::linked_vectors::{LinkedVector, LVNodeIndex};
use crate::{get_orders};
use crate::resource::{Company, Time, HALF_HOUR};
use crate::simulated_annealing::day::TimeOfDay;
use crate::simulated_annealing::neighbor_move::add_new_order::AddNewOrder;
use crate::simulated_annealing::neighbor_move::evaluation_helper::{time_between_three_nodes, time_between_two_nodes};
use crate::simulated_annealing::order_day_flags::OrderFlags;
use crate::simulated_annealing::route::OrderIndex;
use crate::simulated_annealing::neighbor_move::neighbor_move_trait::{CostChange, NeighborMove, ScoreChange};
use crate::simulated_annealing::simulated_annealing::TruckEnum;
use crate::simulated_annealing::week::{DayEnum, Week};

/// This will add an order to a random route where it is allowed to add it to.
/// If you try to add an order, that doesn't have any allowed routes, it panics
pub struct AddMultipleNewOrders {
    where_to_add_orders: Vec<OrderToAdd>,
    order_index: OrderIndex
}

struct OrderToAdd{
    truck_enum: TruckEnum,
    day: DayEnum,
    time_of_day: TimeOfDay,
    insert_after_index: LVNodeIndex,
}
/// This cannot coexist with the normal Add operation.
impl AddMultipleNewOrders {
    pub fn new<R: Rng+?Sized>(truck1: &Week, truck2: &Week, rng: &mut R, order_index: OrderIndex) ->  Option<Self>{
        let order = &get_orders()[order_index];
        let mut orders_to_add: Vec<OrderToAdd> = Vec::new();
        let mut flags = 0;
        for _ in 0..order.frequency as u8 {
            if let Some(order_to_add) = Self::get_random_allowed_order(truck1, truck2, rng, flags, order) {
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

    fn get_random_allowed_order<R: Rng + ?Sized>(truck1: &Week, truck2: &Week, rng: &mut R, flags: u8, order: &Company) -> Option<OrderToAdd>{

        let truck_enum: TruckEnum = rng.random();
        let truck = if truck_enum == TruckEnum::Truck1 {truck1} else {truck2};

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


                return Some(OrderToAdd {
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


    #[cfg(debug_assertions)]
    fn calculate_time_difference(&self, truck1: &Week, truck2: &Week, order_to_add: &OrderToAdd) -> Time{
        let orders = get_orders();
        let order = &orders[self.order_index];
        let route = (if order_to_add.truck_enum == TruckEnum::Truck1 { truck1 } else {truck2}).get(order_to_add.day).get(order_to_add.time_of_day);

        let time = route.calculate_add_order(order_to_add.insert_after_index, self.order_index);
        time + order.emptying_time
    }
}


impl NeighborMove for AddMultipleNewOrders {
    fn evaluate(&self, truck1: &Week, truck2: &Week, order_flags: &OrderFlags) -> CostChange {
        self.where_to_add_orders
            .iter()
            .map(|order_info| self.calculate_time_difference(truck1, truck2, order_info))
            .sum::<Time>()
            // We can always subtract the penalty, becuase this operation will add the order on as many days as needed to mee the frequency requirement.
            - get_orders()[self.order_index].penalty()
    }

    fn apply(&self, truck1: &mut Week, truck2: &mut Week, order_flags: &mut OrderFlags) -> ScoreChange {

        let mut total_score_change = 0;
        for order_info in &self.where_to_add_orders {
            let route= (if order_info.truck_enum == TruckEnum::Truck1 {truck1} else {truck2})
                .get_mut(order_info.day)
                .get_mut(order_info.time_of_day);

            total_score_change += route.apply_add_order(order_info.insert_after_index, self.order_index);
            order_flags.add_order(self.order_index, order_info.day);

            if route.linked_vector.len() == 3{
                total_score_change += HALF_HOUR;
            };
        }

        // We can always subtract the penalty, becuase this operation will add the order on as many days as needed to mee the frequency requirement.
        total_score_change - get_orders()[self.order_index].penalty()
    }
}
