use crate::datastructures::linked_vectors::{LVNodeIndex, LinkedVector};
use crate::get_orders;
use crate::resource::{FULL_DAY, HALF_HOUR, Time};
use crate::simulated_annealing::day::TimeOfDay;
use crate::simulated_annealing::neighbor_move::evaluation_helper::{
    time_between_three_nodes, time_between_two_nodes,
};
use crate::simulated_annealing::neighbor_move::neighbor_move_trait::{CostChange, NeighborMove};
use crate::simulated_annealing::order_day_flags::OrderFlags;
use crate::simulated_annealing::route::{OrderIndex, Route};
use crate::simulated_annealing::simulated_annealing::TruckEnum;
use crate::simulated_annealing::week::{DayEnum, Week};
use rand::Rng;

pub struct ShiftBetweenDays {
    shift: TruckDayTimeNode,
    target: TruckDayTimeNode,
}

pub struct TruckDayTimeNode {
    truck: TruckEnum,
    day: DayEnum,
    time_of_day: TimeOfDay,
    node_index: LVNodeIndex,
    order: OrderIndex,
}

impl ShiftBetweenDays {
    pub fn new<R: Rng + ?Sized>(
        truck1: &Week,
        truck2: &Week,
        rng: &mut R,
        order_flags: &OrderFlags,
    ) -> Option<Self> {
        let shift = Self::get_random_truck_day_time_node(
            truck1,
            truck2,
            rng,
            |r: &Route, i: LVNodeIndex| {
                r.linked_vector.get_tail_index().unwrap() != i
                    && r.linked_vector.get_head_index().unwrap() != i
            },
            None,
        )?;

        let target = Self::get_random_truck_day_time_node(
            truck1,
            truck2,
            rng,
            |r: &Route, i: LVNodeIndex| r.linked_vector.get_tail_index().unwrap() != i,
            Some((shift.order, shift.day, order_flags)),
        )?;

        if shift.truck == target.truck && // if same truck
            shift.day == target.day && // and same day
            shift.time_of_day == target.time_of_day
        // and same time of day (or same route)
        {
            None
        } else {
            Some(ShiftBetweenDays { shift, target })
        }
    }

    fn get_random_truck_day_time_node<R: Rng + ?Sized>(
        truck1: &Week,
        truck2: &Week,
        rng: &mut R,
        requirement: fn(&Route, LVNodeIndex) -> bool,
        shift: Option<(OrderIndex, DayEnum, &OrderFlags)>,
    ) -> Option<TruckDayTimeNode> {
        let truck_enum: TruckEnum = rng.random();
        let truck = if truck_enum == TruckEnum::Truck1 {
            truck1
        } else {
            truck2
        };

        // if this is the random day we want to shift to, check if there are options
        let day: DayEnum = if let Some((shift_order, day_enum, flags)) = shift {
            flags.get_random_day_to_shift_to(shift_order, day_enum, rng)?
        } else {
            // if it is the first random node we try to get, just pick it from a random day
            rng.random()
        };
        let time_of_day: TimeOfDay = rng.random();

        let route = truck.get(day).get(time_of_day);

        let (node, _) = route.linked_vector.get_random(rng).unwrap();

        if requirement(route, node) {
            if let Some((shift_order, _, _)) = shift {
                let orders = get_orders();
                if route.capacity + orders[shift_order].trash() > 100_000 {
                    return None;
                }
            }
            Some(TruckDayTimeNode {
                truck: truck_enum,
                day,
                time_of_day,
                node_index: node,
                order: *route.linked_vector.get_value(node).unwrap(),
            })
        } else {
            None
        }
    }

    /// returns a tuple where item1 contains change in time to shiftRoute, item2 contains change to targetRoute
    fn evaluate_shift_neighbors(&self, truck1: &Week, truck2: &Week) -> Option<(Time/*shift*/, Time/*target*/)> {

        let shift_route = (if self.shift.truck == TruckEnum::Truck1 {
            truck1
        } else {
            truck2
        })
            .get(self.shift.day)
            .get(self.shift.time_of_day);

        let shift_diff = shift_route.calculate_remove_node(self.shift.node_index);

        let target_route = (if self.target.truck == TruckEnum::Truck1 {
            truck1
        } else {
            truck2
        })
            .get(self.target.day)
            .get(self.target.time_of_day);

        let target_diff = target_route.calculate_add_order(self.target.node_index, self.shift.order);

        #[cfg(debug_assertions)]
        {
            let shift_lv = &(if self.shift.truck == TruckEnum::Truck1 {
                truck1
            } else {
                truck2
            })
                .get(self.shift.day)
                .get(self.shift.time_of_day)
                .linked_vector;

            let orders = get_orders();
            let emptying_time = orders[*shift_lv.get_value_unsafe(self.shift.node_index)].emptying_time;

            let before_shift = orders[*shift_lv.get_prev_value_unsafe(self.shift.node_index)].matrix_id;

            let shift = orders[*shift_lv.get_value_unsafe(self.shift.node_index)].matrix_id;
            let after_shift = orders[*shift_lv.get_next_value_unsafe(self.shift.node_index)].matrix_id;

            let target_lv = &(if self.target.truck == TruckEnum::Truck1 {
                truck1
            } else {
                truck2
            })
                .get(self.target.day)
                .get(self.target.time_of_day)
                .linked_vector;
            let t1 = orders[*target_lv.get_value_unsafe(self.target.node_index)].matrix_id;
            let t2 = orders[*target_lv.get_next_value_unsafe(self.target.node_index)].matrix_id;

            // add the difference between the shifting_node and the two nodes where it will be put between
            let mut old_target_diff = time_between_three_nodes(t1, shift, t2);
            // remove the time between these two nodes
            old_target_diff -= time_between_two_nodes(t1, t2);
            old_target_diff += emptying_time;

            // add the time between the two neighbors of the node that will be shifted
            let mut old_shift_diff = time_between_two_nodes(before_shift, after_shift);
            // remove the time between the node that will be shifted and it's current neighbors
            old_shift_diff -= time_between_three_nodes(before_shift, shift, after_shift);
            old_shift_diff -= emptying_time;
            assert_eq!(shift_diff, old_shift_diff);
            assert_eq!(target_diff, old_target_diff);
        }

        Some((shift_diff, target_diff))
    }

    fn apply_same_truck_case(
        &self,
        truck: &mut Week,
        order_flags: &mut OrderFlags,
        shift_diff: Time,
        target_diff: Time,
    ) -> Time {
        let shift_route = truck
            .get_mut(self.shift.day)
            .get_mut(self.shift.time_of_day);

        shift_route.apply_remove_node(self.shift.node_index);
        order_flags.remove_order(self.shift.order, self.shift.day);
        let shift_route_empty: Time = if shift_route.linked_vector.len() == 2 {
            -HALF_HOUR
        } else {
            0
        };

        let target_route = truck
            .get_mut(self.target.day)
            .get_mut(self.target.time_of_day);

        target_route.apply_add_order(self.target.node_index, self.shift.order);

        order_flags.add_order(self.shift.order, self.target.day);

        let target_diff_new_route: Time = if target_route.linked_vector.len() == 3 {
            HALF_HOUR
        } else {
            0
        };

        shift_diff + target_diff + shift_route_empty + target_diff_new_route
    }
}

impl NeighborMove for ShiftBetweenDays {
    fn evaluate(&self, truck1: &Week, truck2: &Week, _: &OrderFlags) -> Option<CostChange> {
        // this is the time difference
        let (shift_diff, target_diff) = self.evaluate_shift_neighbors(truck1, truck2)?;

        let target_day = (if self.target.truck == TruckEnum::Truck1 {
            truck1
        } else {
            truck2
        })
        .get(self.target.day);
        if target_day.get_total_time() + target_diff > FULL_DAY {
            let overtime = ((target_day.get_total_time() + target_diff - FULL_DAY) * /*penalty percentage*/3)
                / 100;
            return Some(shift_diff + target_diff + overtime);
        }

        Some(shift_diff + target_diff)
    }

    fn apply(&self, truck1: &mut Week, truck2: &mut Week, order_flags: &mut OrderFlags) -> Time {
        let (shift_diff, target_diff) = self.evaluate_shift_neighbors(truck1, truck2).unwrap();
        let (shift_route, target_route): (&mut Route, &mut Route) =
            match (self.shift.truck, self.target.truck) {
                (TruckEnum::Truck1, TruckEnum::Truck2) => (
                    truck1
                        .get_mut(self.shift.day)
                        .get_mut(self.shift.time_of_day),
                    truck2
                        .get_mut(self.target.day)
                        .get_mut(self.target.time_of_day),
                ),
                (TruckEnum::Truck2, TruckEnum::Truck1) => (
                    truck2
                        .get_mut(self.shift.day)
                        .get_mut(self.shift.time_of_day),
                    truck1
                        .get_mut(self.target.day)
                        .get_mut(self.target.time_of_day),
                ),
                (t1, t2) if t1 == t2 => {
                    let truck = if t1 == TruckEnum::Truck1 {
                        truck1
                    } else {
                        truck2
                    };
                    return self.apply_same_truck_case(truck, order_flags, shift_diff, target_diff);
                }
                _ => unreachable!(),
            };

        shift_route.apply_remove_node(self.shift.node_index);
        target_route.apply_add_order(self.target.node_index, self.shift.order);

        order_flags.remove_order(self.shift.order, self.shift.day);
        order_flags.add_order(self.shift.order, self.target.day);

        let shift_route_empty: Time = if shift_route.linked_vector.len() == 2 {
            -HALF_HOUR
        } else {
            0
        };
        let target_diff_new_route: Time = if target_route.linked_vector.len() == 3 {
            HALF_HOUR
        } else {
            0
        };

        shift_diff + target_diff + shift_route_empty + target_diff_new_route
    }
}
