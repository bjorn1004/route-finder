use rand::Rng;
use crate::datastructures::linked_vectors::{LVNodeIndex, LinkedVector};
use crate::get_orders;
use crate::resource::{HALF_HOUR};
use crate::simulated_annealing::day::TimeOfDay;
use crate::simulated_annealing::neighbor_move::neighbor_move_trait::{CostChange, NeighborMove, ScoreChange};
use crate::simulated_annealing::order_day_flags::OrderFlags;
use crate::simulated_annealing::route::OrderIndex;
use crate::simulated_annealing::simulated_annealing::TruckEnum;
use crate::simulated_annealing::solution::Solution;
use crate::simulated_annealing::week::{DayEnum};

pub struct RemoveOrder{
    truck_enum: TruckEnum,
    day_enum: DayEnum,
    time_of_day: TimeOfDay,
    node_index: LVNodeIndex,
    pub order_index: OrderIndex,
}

impl RemoveOrder{
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

            return Some((RemoveOrder{
                truck_enum,
                day_enum,
                time_of_day,
                node_index,
                order_index: *order_index,
            },
            *order_index))
        }
        panic!("The linkedvector was completely empty when trying to remove an order");
    }
}

impl NeighborMove for RemoveOrder {
    fn evaluate(&self, solution: &Solution) -> CostChange {
        let route = (if self.truck_enum == TruckEnum::Truck1 {&solution.truck1} else {&solution.truck2})
            .get(self.day_enum)
            .get(self.time_of_day);

        let diff = route.calculate_remove_node(self.node_index);
        let empty_route = if route.linked_vector.len() == 3 {
            -HALF_HOUR
        } else {
            0
        };

        let order = &get_orders()[self.order_index];
        let penalty = if solution.order_flags.get_filled_count(self.order_index) == order.frequency as u32{
            order.penalty()
        } else {
            0
        };


        diff + empty_route + penalty
    }

    fn apply(&self, solution: &mut Solution) -> ScoreChange {

        let route = (if self.truck_enum == TruckEnum::Truck1 {&mut solution.truck1} else {&mut  solution.truck2})
            .get_mut(self.day_enum)
            .get_mut(self.time_of_day);

        let diff = route.apply_remove_node(self.node_index);

        let empty_route = if route.linked_vector.len() == 2 {
            -HALF_HOUR
        } else {
            0
        };

        let order = &get_orders()[self.order_index];
        let penalty = if solution.order_flags.get_filled_count(self.order_index) == order.frequency as u32{
            order.penalty()
        } else {
            0
        };
        solution.order_flags.remove_order(self.order_index, self.day_enum);
        diff + empty_route + penalty
    }
}