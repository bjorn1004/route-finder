use rand::Rng;
use crate::datastructures::linked_vectors::{LinkedVector};
use crate::get_orders;
use crate::simulated_annealing::day::TimeOfDay;
use crate::simulated_annealing::neighbor_move::evaluation_helper::{calculate_capacity_overflow, calculate_time_overflow};
use crate::simulated_annealing::neighbor_move::evaluation::Evaluation;
use crate::simulated_annealing::neighbor_move::neighbor_move_trait::{NeighborMove, ScoreChange};
use crate::simulated_annealing::order_location::{ExactLocation, OrderInfo};
use crate::simulated_annealing::route::{OrderIndex};
use crate::simulated_annealing::simulated_annealing::TruckEnum;
use crate::simulated_annealing::solution::Solution;
use crate::simulated_annealing::week::{DayEnum};

pub struct RemoveMultipleOrders{
    orders_to_remove: Vec<ExactLocation>,
    order_index: OrderIndex,
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

            let order_loc = solution.order_location.get_order_location(*order_index);

            let orders_to_remove = match order_loc {
                OrderInfo::Empty => unreachable!(),
                OrderInfo::Filled =>
                vec![ExactLocation{
                    truck_enum,
                    day_enum,
                    time_of_day,
                    node_index
                }],
                OrderInfo::Twice(locs) => { locs.to_vec() }
                OrderInfo::Thrice(locs) => { locs.to_vec() }
                OrderInfo::Frice(locs) => { locs.to_vec() }
            };

            return Some((RemoveMultipleOrders{
                orders_to_remove,
                order_index: *order_index,
            }, *order_index));
        }
        panic!("The linkedvector was completely empty when trying to remove an order");
    }
}

impl NeighborMove for RemoveMultipleOrders {
    fn evaluate(&self, solution: &Solution) -> Evaluation {
        let order = &get_orders()[self.order_index];

        let evaluation: Evaluation = self.orders_to_remove.iter()
            .map(|order_info|{
                let day = if order_info.truck_enum == TruckEnum::Truck1 {&solution.truck1} else {&solution.truck2}.get(order_info.day_enum);
                let route = day.get(order_info.time_of_day);

                let time_diff = route.calculate_remove_node(order_info.node_index);

                let time_overflow_delta =
                    calculate_time_overflow(time_diff, day.get_total_time());

                let capacity_overflow_delta =
                    calculate_capacity_overflow(-(order.total_container_volume as i32), route.capacity as i32);

                Evaluation{
                    cost: time_diff,
                    time_overflow_delta,
                    capacity_overflow_delta,
                }
            })
            .sum();

        Evaluation{
            cost: evaluation.cost + get_orders()[self.order_index].penalty,
            ..evaluation
        }
    }

    fn apply(&self, solution: &mut Solution) -> ScoreChange {
        let mut total_change = 0;

        let mut truck1 = solution.truck1.get_all_as_mut();
        let mut truck2 = solution.truck2.get_all_as_mut();
        let mut compact_changes = vec![];

        for order_info in &self.orders_to_remove{
            if order_info.truck_enum == TruckEnum::Truck1 {
                let (score_change, compact_change) = truck1[order_info.day_enum as usize * 2 + order_info.time_of_day as usize].apply_remove_node(order_info.node_index);
                compact_changes.push((compact_change[0].0, compact_change[0].1, order_info.day_enum));
                total_change += score_change
            } else {
                let (score_change, compact_change) = truck2[order_info.day_enum as usize * 2 + order_info.time_of_day as usize].apply_remove_node(order_info.node_index);
                compact_changes.push((compact_change[0].0, compact_change[0].1, order_info.day_enum));
                total_change += score_change
            }
        }


        solution.order_location.update_shifts_in_route(compact_changes);
        solution.order_location.remove_order(self.order_index);

        total_change + get_orders()[self.order_index].penalty
    }
}
