use petgraph::visit::NodeIndexable;
use crate::datastructures::compact_linked_vector::CompactLinkedVector;
use crate::datastructures::linked_vectors::LinkedVector;
use crate::{get_distance_matrix, get_orders};
use crate::resource::MatrixID;
use crate::simulated_annealing::neighbor_move::evaluation_helper::time_between_two_nodes;

#[derive(Debug, Clone)]
pub struct Route{
    pub linked_vector: CompactLinkedVector<OrderIndex>,
    pub capacity: u64,
    pub time: f32,
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
            time: 30f32*60f32,
        }
    }

    pub fn check_correctness_trash(&self){
        let orders = get_orders();
        assert_eq!(self.linked_vector
                       .iter()
                       .map(|(_, matrix_id)| orders[*matrix_id as usize].container_volume as u64)
                       .sum::<u64>(),
                   0u64, "The currently stored trash volume is incorrect")

    }
    pub fn check_correctness_time(&self) -> bool{
        let dist = get_distance_matrix();
        let orders = get_orders();
        let mut time_travel = 0f32;
        let lv = &self.linked_vector;
        for (node_i, order_i) in lv.iter().skip(1){
            let prev_order_i = lv.get_prev_value(node_i).unwrap();

            let node_mi = dist.from_index(orders[*order_i].matrix_id as usize);
            let prev_node_mi = dist.from_index(orders[*prev_order_i].matrix_id as usize);

            time_travel += time_between_two_nodes(node_mi, prev_node_mi);

            if node_i == lv.get_tail_index().unwrap(){
                continue;
            }
            time_travel += orders[*order_i].emptying_time;

        }

        time_travel += 30f32 * 60f32; // add the 30 minutes of trash dumping
        let difference = self.time - time_travel;
        if difference > 1f32{
            panic!();
            return false
        }
        true
    }



}