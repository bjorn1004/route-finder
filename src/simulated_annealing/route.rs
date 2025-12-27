use crate::datastructures::compact_linked_vector::CompactLinkedVector;
use crate::datastructures::linked_vectors::{LVNodeIndex, LinkedVector};
use crate::{get_orders, EXTREME_TEST_FLAG};
use crate::resource::{HALF_HOUR, Time};
use crate::simulated_annealing::neighbor_move::evaluation_helper::{time_between_three_nodes, time_between_two_nodes};

#[derive(Debug, Clone)]
pub struct Route {
    pub linked_vector: CompactLinkedVector<OrderIndex>,
    pub capacity: u32,
    pub time: Time,
}
pub type OrderIndex = usize;

impl Route {
    /// Construct an empty route
    /// We should maybe add the dropoff location as the first and last element of this list?
    pub fn new() -> Self {
        let mut route: CompactLinkedVector<OrderIndex> = CompactLinkedVector::new();
        route.push_back(get_orders().len() - 1);
        route.push_back(get_orders().len() - 1);
        Route {
            linked_vector: route,
            capacity: 0,
            time: HALF_HOUR,
        }
    }

    pub fn check_correctness_trash(&self) {
        let orders = get_orders();
        assert_eq!(
            self.linked_vector
                .iter()
                .map(|(_, matrix_id)| orders[*matrix_id].container_volume as u64)
                .sum::<u64>(),
            0u64,
            "The currently stored trash volume is incorrect"
        )
    }

    pub fn recalculate_total_time(&mut self) {
        self.time = self.calculate_time();
    }
    pub fn check_correctness_time(&self) -> bool {
        if !EXTREME_TEST_FLAG{
            return true
        }
        let calculated_time = self.calculate_time();
        let difference = self.time - calculated_time;
        if difference > 1 {
            if self.linked_vector.len() == 2 && calculated_time == HALF_HOUR {
                return true;
            }
            println!("found inconsistency");
            println!("route length: {}", self.linked_vector.len());
            println!("stored time: {}", self.time);
            println!("actual time: {}", calculated_time);
            return false;
        }
        true
    }

    /// This function calculates how much time the route takes.
    /// It always adds the 30 minutes dropoff time at the end of the route, even if it doesn't have to.
    /// This is to stay consistent with how we store the Time value in the route.
    fn calculate_time(&self) -> Time {
        let orders = get_orders();
        let mut time_travel = 0;
        let lv = &self.linked_vector;
        for (node_i, order_i) in lv.iter() {
            if lv.get_tail_index() == Some(node_i) {
                break;
            }
            let matrix_i = orders[*order_i].matrix_id;
            let next_matrix_i = orders[*lv.get_next_value(node_i).unwrap()].matrix_id;

            time_travel += time_between_two_nodes(matrix_i, next_matrix_i);
            time_travel += orders[*order_i].emptying_time;

            // let prev_order_i = lv.get_prev_value(node_i).unwrap();
            //
            // let node_mi = orders[*order_i].matrix_id.into();
            // let prev_node_mi = orders[*prev_order_i].matrix_id.into();
            //
            // time_travel += time_between_two_nodes(prev_node_mi, node_mi);
            // time_travel += orders[*order_i].emptying_time;
        }

        // Add the 30 minutes for the dropoff
        time_travel += HALF_HOUR;
        time_travel
    }
    pub fn calculate_remove_node(&self, node: LVNodeIndex) -> Time{
        let orders = get_orders();
        let lv = &self.linked_vector;
        let order = &orders[*lv.get_value_unsafe(node)];

        let prev = orders[*lv.get_prev_value_unsafe(node)].matrix_id;
        let middle = order.matrix_id;
        let next = orders[*lv.get_next_value_unsafe(node)].matrix_id;

        let time_diff =
            - time_between_three_nodes(prev, middle, next)
                + time_between_two_nodes(prev, next);

        time_diff - order.emptying_time
    }
    pub fn apply_remove_node(&mut self, node: LVNodeIndex) -> Time{
        let orders = get_orders();
        let lv = &mut self.linked_vector;
        let order = &orders[*lv.get_value_unsafe(node)];

        let prev = orders[*lv.get_prev_value_unsafe(node)].matrix_id;
        let middle = order.matrix_id;
        let next = orders[*lv.get_next_value_unsafe(node)].matrix_id;

        let time_diff =
            - time_between_three_nodes(prev, middle, next)
            + time_between_two_nodes(prev, next)
            - order.emptying_time;

        self.time += time_diff;
        self.capacity -= order.total_container_volume;
        lv.remove(node);
        lv.compact();
        time_diff
    }
    /// a special function for the FIXPLZPLZPLZPLZPLZPLZPLZ function.
    pub fn apply_remove_node_without_compact(&mut self, node: LVNodeIndex) -> Time{
        let orders = get_orders();
        let lv = &mut self.linked_vector;
        let order = &orders[*lv.get_value_unsafe(node)];

        let prev = orders[*lv.get_prev_value_unsafe(node)].matrix_id;
        let middle = order.matrix_id;
        let next = orders[*lv.get_next_value_unsafe(node)].matrix_id;

        let time_diff =
            - time_between_three_nodes(prev, middle, next)
            + time_between_two_nodes(prev, next)
            - order.emptying_time;

        self.time += time_diff;
        self.capacity -= order.total_container_volume;
        lv.remove(node);
        time_diff
    }
    pub fn calculate_add_order(&self, insert_after_this: LVNodeIndex, order_to_insert: OrderIndex) -> Time {
        let orders = get_orders();
        let lv = &self.linked_vector;

        let order = &orders[order_to_insert];

        let prev = orders[*lv.get_value_unsafe(insert_after_this)].matrix_id;
        let middle = order.matrix_id;
        let next = orders[*lv.get_next_value_unsafe(insert_after_this)].matrix_id;

        let time_diff = time_between_three_nodes(prev, middle, next)
            - time_between_two_nodes(prev, next);

        time_diff + order.emptying_time
    }
    pub fn apply_add_order(&mut self, insert_after_this: LVNodeIndex, order_index: OrderIndex) -> Time {
        let orders = get_orders();
        let lv = &mut self.linked_vector;

        let order = &orders[order_index];

        let prev = orders[*lv.get_value_unsafe(insert_after_this)].matrix_id;
        let middle = order.matrix_id;
        let next = orders[*lv.get_next_value_unsafe(insert_after_this)].matrix_id;

        let time_diff =
              time_between_three_nodes(prev, middle, next)
            - time_between_two_nodes(prev, next)
            + order.emptying_time;

        self.time += time_diff;
        self.capacity += order.total_container_volume;
        lv.insert_after(insert_after_this, order_index);

        time_diff
    }
}

impl Default for Route {
    fn default() -> Self {
        Self::new()
    }
}


#[cfg(test)]
use test_env_helpers::*;

#[before_all]
#[cfg(test)]
mod tests {
    use crate::{get_distance_matrix, get_orders, DISTANCE_MATRIX, ORDERS};
    use crate::parser::{parse_distance_matrix, parse_orderfile};
    use crate::resource::{Company, Frequency};
    use crate::simulated_annealing::route::Route;

    fn before_all(){
        // We make most of the frequencies 0 to make the penalty score a lot lower.
        // This helps with
        let mut order_vec: Vec<Company>= parse_orderfile() .unwrap();
        for (i, order) in order_vec.iter_mut().enumerate() {
            if i > 3 {
                order.frequency = Frequency::None;
            }
        }
        ORDERS.set(order_vec.into()).ok();
        let distance_matrix = parse_distance_matrix().unwrap();
        DISTANCE_MATRIX.set(distance_matrix).ok();
    }

    #[test]
    fn initialization_check(){
        let orders = get_orders();
        println!("{}", orders[0].order);
        let matrix = get_distance_matrix();
        println!("{}", matrix.get_edge_weight(0.into(), 1.into()).unwrap())
    }

    #[test]
    fn single_add(){
        let route = &mut Route::default();

        route.apply_add_order(0, 0);


        let before_recalc = route.time;
        route.recalculate_total_time();
        assert_eq!(before_recalc, route.time);
    }

    #[test]
    fn single_remove(){
        let route = &mut Route::default();

        let before_time = route.calculate_time();

        route.apply_add_order(0, 0);
        route.apply_remove_node(2);


        assert_eq!(before_time, route.time);
        route.recalculate_total_time();
        assert_eq!(before_time, route.time);
    }
}