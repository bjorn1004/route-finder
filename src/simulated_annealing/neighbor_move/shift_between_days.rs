use petgraph::visit::NodeIndexable;
use rand::Rng;
use crate::datastructures::linked_vectors::{LinkedVector, LVNodeIndex};
use crate::{get_distance_matrix, get_orders};
use crate::simulated_annealing::day::{Day, TimeOfDay};
use crate::simulated_annealing::neighbor_move::evaluation_helper::{time_between_three_nodes, time_between_two_nodes};
use crate::simulated_annealing::neighbor_move::neighbor_move_trait::{CostChange, NeighborMove};
use crate::simulated_annealing::order_day_flags::OrderFlags;
use crate::simulated_annealing::route::{OrderIndex, Route};
use crate::simulated_annealing::simulated_annealing::TruckEnum;
use crate::simulated_annealing::week::{DayEnum, Week};

pub struct ShiftBetweenDays {
    shift: TruckDayTimeNode,
    target: TruckDayTimeNode,
}

pub struct TruckDayTimeNode{
    truck: TruckEnum,
    day: DayEnum,
    time_of_day: TimeOfDay,
    node_index: LVNodeIndex,
    order: OrderIndex
}

impl ShiftBetweenDays {
    pub fn new<R: Rng + ?Sized>(truck1: &Week, truck2: &Week, rng: &mut R, order_flags: &OrderFlags) -> Option<Self>{
        let shift =
            Self::get_random_truck_day_time_node(
                truck1,
                truck2,
                rng,
                |r:&Route, i:LVNodeIndex|
                    r.linked_vector.get_tail_index().unwrap() != i && r.linked_vector.get_head_index().unwrap() != i,
                None
            )?;

        let target = Self::get_random_truck_day_time_node(
            truck1,
            truck2,
            rng,
            |r:&Route, i:LVNodeIndex|
                r.linked_vector.get_tail_index().unwrap() != i,
            Some((shift.order, shift.day, order_flags))
        )?;

        if shift.truck == target.truck && // if same truck
            shift.day == target.day && // and same day
            shift.time_of_day == target.time_of_day // and same time of day (or same route)
            {
            None
        } else {
            Some(ShiftBetweenDays {
                shift,
                target,
            })
        }
    }

    fn get_random_truck_day_time_node<R: Rng + ?Sized>(
        truck1: &Week,
        truck2: &Week,
        rng: &mut R,
        requirement: fn(&Route, LVNodeIndex) -> bool,
        shift: Option<(OrderIndex, DayEnum, &OrderFlags)>) -> Option<TruckDayTimeNode>{

        let truck_enum:TruckEnum = rng.random();
        let truck = if truck_enum == TruckEnum::Truck1 {truck1} else {truck2};

        // if this is the random day we want to shift to, check if there are options
        let day:DayEnum = if let Some((shift_order, day_enum, flags)) = shift {
            flags.get_random_day_to_shift_to(shift_order, day_enum, rng)?
        } else {
            // if it is the first random node we try to get, just pick it from a random day
            rng.random()
        };
        let time_of_day:TimeOfDay = rng.random();

        let route = truck.get(day).get(time_of_day);

        let (node, order_index) = route.linked_vector.get_random(rng).unwrap();


        if requirement(route, node){

            if let Some((shift_order, _, _)) = shift{
                let orders = get_orders();
                if route.capacity + orders[shift_order].trash() > 100_000{
                    return None;
                }
            }
            Some(TruckDayTimeNode{
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

    /// returns a tuple where item1 contains change to shiftRoute, item2 contains change to targetRoute
    fn evaluate_shift_neighbors(&self, truck1: &Week, truck2: &Week) -> Option<(CostChange, CostChange)>{
        let shift_lv = &(if self.shift.truck == TruckEnum::Truck1 {truck1} else {truck2})
            .get(self.shift.day)
            .get(self.shift.time_of_day)
            .linked_vector;

        let orders = get_orders();
        let dist = get_distance_matrix();

        let before_shift = dist.from_index(orders[*shift_lv.get_prev_value(self.shift.node_index).unwrap()].matrix_id as usize);
        let shift = dist.from_index(orders[*shift_lv.get_value(self.shift.node_index).unwrap()].matrix_id as usize);
        let after_shift = dist.from_index(orders[*shift_lv.get_next_value(self.shift.node_index).unwrap()].matrix_id as usize);

        let target_lv = &(if self.target.truck == TruckEnum::Truck1 {truck1} else {truck2})
            .get(self.target.day)
            .get(self.target.time_of_day)
            .linked_vector;
        let t1 = dist.from_index(orders[*target_lv.get_value(self.target.node_index).unwrap()].matrix_id as usize);
        let t2 = dist.from_index(orders[*target_lv.get_next_value(self.target.node_index).unwrap()].matrix_id as usize);

        // add the difference between the shifting_node and the two nodes where it fill be put between
        let mut target_diff = time_between_three_nodes(t1, shift, t2);
        // remove the time between these two nodes
        target_diff -= time_between_two_nodes(t1, t2);


        // add the time between the two neighbors of the node that will be shifted
        let mut shift_diff = time_between_two_nodes(before_shift, after_shift);
        // remove the time between the node that will be shifted and it's current neighbors
        shift_diff -= time_between_three_nodes(before_shift, shift, after_shift);

        Some((shift_diff, target_diff))
    }

    fn apply_same_truck_case(&self, truck: &mut Week, order_flags: &mut OrderFlags, shift_diff: CostChange, target_diff: CostChange){
        let order = &get_orders()[self.shift.order];
        let shift_route= truck.get_mut(self.shift.day).get_mut(self.shift.time_of_day);

        let shift_value = *shift_route.linked_vector.get_value(self.shift.node_index).unwrap();

        shift_route.capacity -= order.trash();
        shift_route.time += shift_diff;

        shift_route.linked_vector.remove(self.shift.node_index);
        shift_route.linked_vector.compact();

        let target_route = truck.get_mut(self.target.day).get_mut(self.target.time_of_day);

        target_route.capacity += order.trash();
        target_route.time += target_diff;

        target_route.linked_vector.insert_after(self.target.node_index, shift_value);

        order_flags.remove_order(self.shift.order, self.shift.day);
        order_flags.add_order(self.shift.order, self.target.day);
    }
}

impl NeighborMove for ShiftBetweenDays {
    fn evaluate(&self, truck1: &Week, truck2: &Week, order_flags: &OrderFlags) -> Option<CostChange> {
        // this is the time difference
        let (shift_diff, target_diff) = self.evaluate_shift_neighbors(truck1, truck2)?;

        let target_day = (if self.target.truck == TruckEnum::Truck1 {truck1} else {truck2})
            .get(self.target.day);
        if target_day.get_time() + target_diff > 12f32 * 60f32 * 60f32{
            let overtime = (target_day.get_time() + target_diff - 12f32 * 60f32 * 60f32) * 1f32;
            return Some(shift_diff + target_diff + overtime)
        }

        Some(shift_diff+target_diff)
    }

    fn apply(&self, truck1: &mut Week, truck2: &mut Week, order_flags: &mut OrderFlags) {


        let (shift_diff, target_diff) = self.evaluate_shift_neighbors(truck1, truck2).unwrap();
        let (shift_route, target_route): (&mut Route, &mut Route) = match (self.shift.truck, self.target.truck) {
            (TruckEnum::Truck1, TruckEnum::Truck2) => (truck1.get_mut(self.shift.day).get_mut(self.shift.time_of_day),
                                                       truck2.get_mut(self.target.day).get_mut(self.target.time_of_day)),
            (TruckEnum::Truck2, TruckEnum::Truck1) => (truck2.get_mut(self.shift.day).get_mut(self.shift.time_of_day),
                                                       truck1.get_mut(self.target.day).get_mut(self.target.time_of_day)),
            (t1, t2) if t1 == t2 => {
                let truck = if t1 == TruckEnum::Truck1 {truck1} else {truck2};
                self.apply_same_truck_case(truck, order_flags, shift_diff, target_diff);
                return
            }
            _ => unreachable!(),
        };

        let order = &get_orders()[self.shift.order];

        let shift_value = *shift_route.linked_vector.get_value(self.shift.node_index).unwrap();

        let old_shift_len = shift_route.linked_vector.len();
        shift_route.capacity -= order.trash();
        shift_route.time += shift_diff;

        shift_route.linked_vector.remove(self.shift.node_index);
        shift_route.linked_vector.compact();
        let new_shift_len = shift_route.linked_vector.len();

        let old_target_len = target_route.linked_vector.len();
        target_route.capacity += order.trash();
        target_route.time += target_diff;

        target_route.linked_vector.insert_after(self.target.node_index, shift_value);
        let new_target_len = target_route.linked_vector.len();

        order_flags.remove_order(self.shift.order, self.shift.day);
        order_flags.add_order(self.shift.order, self.target.day);

        assert_eq!(old_shift_len-1, new_shift_len);
        assert_eq!(old_target_len+1, new_target_len);
        target_route.check_correctness_time();
        shift_route.check_correctness_time();
    }
}