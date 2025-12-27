use rand::Rng;
use crate::datastructures::linked_vectors::{LVNodeIndex, LinkedVector};
use crate::get_orders;
use crate::resource::{Time, HALF_HOUR};
use crate::simulated_annealing::day::TimeOfDay;
use crate::simulated_annealing::neighbor_move::neighbor_move_trait::{CostChange, NeighborMove, ScoreChange};
use crate::simulated_annealing::route::{OrderIndex, Route};
use crate::simulated_annealing::simulated_annealing::TruckEnum;
use crate::simulated_annealing::solution::Solution;
use crate::simulated_annealing::week::{DayEnum, Week};

pub struct RemoveMultipleOrders{
    orders_to_remove: Vec<RemoveOrderInfo>,
    pub order_index: OrderIndex,
}

struct RemoveOrderInfo {
    truck_enum: TruckEnum,
    day_enum: DayEnum,
    time_of_day: TimeOfDay,
    node_index: LVNodeIndex
}

impl RemoveMultipleOrders{
    pub fn new<R: Rng + ?Sized>(solution: &Solution, rng: &mut R) -> Option<(Self, OrderIndex)>{
        let truck_enum: TruckEnum = rng.random();
        let day_enum: DayEnum = rng.random();
        let time_of_day: TimeOfDay = rng.random();

        let route = (if truck_enum == TruckEnum::Truck1 {&solution.truck1} else {&solution.truck2})
            .get(day_enum)
            .get(time_of_day);


        let lv = &route.linked_vector;
        if lv.len() == 2 {
            return None;
        }

        while let Some((node_index, order_index)) = lv.get_random(rng) {
            if node_index == lv.get_head_index()? || node_index == lv.get_tail_index()? {
                continue;
            }

            let mut orders_to_remove = vec![
                RemoveOrderInfo{
                    truck_enum,
                    day_enum,
                    time_of_day,
                    node_index
                }
            ];

            let unremoved_days = solution.order_flags.get_other_days_of_an_order(*order_index, day_enum);

            for other_day in unremoved_days{
                orders_to_remove.push(Self::find_other_day(&solution, other_day, *order_index))

            }
            return Some((RemoveMultipleOrders{
                orders_to_remove,
                order_index: *order_index,
            }, *order_index));
        }
        panic!("The linkedvector was completely empty when trying to remove an order");
    }

    fn find_other_day(solution: &Solution, day_enum: DayEnum, order_index: OrderIndex) -> RemoveOrderInfo{
        if let Some((time_of_day, node_index)) = Self::find_other_day_in_truck(&solution.truck1, day_enum, order_index) {
            return RemoveOrderInfo{
                truck_enum: TruckEnum::Truck1,
                day_enum,
                time_of_day,
                node_index
            }
        } else if let Some((time_of_day, node_index)) = Self::find_other_day_in_truck(&solution.truck2, day_enum, order_index) {
            return RemoveOrderInfo{
                truck_enum: TruckEnum::Truck2,
                day_enum,
                time_of_day,
                node_index
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

    fn apply_on_one_route(&self, route: &mut Route, node_index: LVNodeIndex) -> Time {

        let score = route.apply_remove_node(node_index);

        if route.linked_vector.len() == 2 {
            return score - HALF_HOUR;
        };

        score
    }
}

impl NeighborMove for RemoveMultipleOrders {
    fn evaluate(&self, solution: &Solution) -> CostChange {
        let order = &get_orders()[self.order_index];
        let mut total_change = 0;

        for order_info in &self.orders_to_remove{
            let route = (if order_info.truck_enum == TruckEnum::Truck1 {&solution.truck1} else {&solution.truck2})
                .get(order_info.day_enum)
                .get(order_info.time_of_day);

            total_change += route.calculate_remove_node(order_info.node_index);

            total_change += if route.linked_vector.len() == 3 {
                -HALF_HOUR
            } else {
                0
            };
        }

        total_change + order.penalty
    }

    fn apply(&self, solution: &mut Solution) -> ScoreChange {

        let mut total_change = 0;
        let mut truck1 = solution.truck1.get_all_as_mut();
        let mut truck2 = solution.truck2.get_all_as_mut();
        for order_info in &self.orders_to_remove{
            if order_info.truck_enum == TruckEnum::Truck1 {
                total_change += self.apply_on_one_route(truck1[order_info.day_enum as usize * 2 + order_info.time_of_day as usize], order_info.node_index);
            } else {
                total_change += self.apply_on_one_route(truck2[order_info.day_enum as usize * 2 + order_info.time_of_day as usize], order_info.node_index);
            }

        }

        solution.order_flags.clear(self.order_index);

        total_change + get_orders()[self.order_index].penalty
    }
}
