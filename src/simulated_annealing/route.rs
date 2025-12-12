use crate::datastructures::compact_linked_vector::CompactLinkedVector;
use crate::datastructures::linked_vectors::{LVNodeIndex, LinkedVector};
use crate::{get_orders};
use crate::resource::{Time, HALF_HOUR};
use crate::simulated_annealing::neighbor_move::evaluation_helper::{time_between_three_nodes, time_between_two_nodes};

#[derive(Debug, Clone)]
pub struct Route{
    pub linked_vector: CompactLinkedVector<OrderIndex>,
    pub capacity: u32,
    pub time: Time,
}
pub type OrderIndex = usize;

impl Route{
    /// Construct an empty route
    /// We should maybe add the dropoff location as the first and last element of this list?
    pub fn new() -> Self{
        let mut route: CompactLinkedVector<OrderIndex>= CompactLinkedVector::new();
        route.push_back(get_orders().len()-1);
        route.push_back(get_orders().len()-1);
        Route{
            linked_vector: route,
            capacity: 0,
            time: HALF_HOUR,
        }
    }

    pub fn check_correctness_trash(&self){
        let orders = get_orders();
        assert_eq!(self.linked_vector
                       .iter()
                       .map(|(_, matrix_id)| orders[*matrix_id].container_volume as u64)
                       .sum::<u64>(),
                   0u64, "The currently stored trash volume is incorrect")

    }

    pub fn recalculate_total_time(&mut self) {
        self.time = self.calculate_time();
    }
    pub fn check_correctness_time(&self, message: &str) -> bool{
        let calculated_time = self.calculate_time();
        let difference = self.time - calculated_time;
        if difference != 0 {
            if self.linked_vector.len() == 2 && calculated_time == HALF_HOUR{
                return true
            }
            println!("found inconsistency");
            println!("{}", message);
            println!("route length: {}", self.linked_vector.len());
            println!("stored time: {}", self.time);
            println!("actual time: {}", calculated_time);
            return false
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
            if lv.get_tail_index() == Some(node_i){
                break;
            }
            let matrix_i = orders[*order_i].matrix_id;
            let next_matrix_i = orders[*lv.get_next_value_unsafe(node_i)].matrix_id;

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

    pub fn remove_node(&mut self, node: LVNodeIndex) -> Time{
        let orders = get_orders();
        let lv = &mut self.linked_vector;
        let order = &orders[*lv.get_value_unsafe(node)];

        let prev = orders[*lv.get_prev_value_unsafe(node)].matrix_id;
        let middle = order.matrix_id;
        let next = orders[*lv.get_next_value_unsafe(node)].matrix_id;

        let time_diff =
            - time_between_three_nodes(prev, middle, next)
            + time_between_two_nodes(prev, next);

        self.time += time_diff - order.emptying_time;
        self.capacity -= order.trash();
        lv.remove(node);
        lv.compact();
        time_diff
    }

    pub fn calculate_time_if_remove_node(&self, node: LVNodeIndex) -> Time {
        let orders = get_orders();
        let lv = &self.linked_vector;
        let order = &orders[*lv.get_value_unsafe(node)];

        let prev = orders[*lv.get_prev_value_unsafe(node)].matrix_id;
        let middle = order.matrix_id;
        let next = orders[*lv.get_next_value_unsafe(node)].matrix_id;

        - time_between_three_nodes(prev, middle, next) 
            + time_between_two_nodes(prev, next) 
            - order.emptying_time
    }
}