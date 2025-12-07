use petgraph::matrix_graph::NodeIndex;
use crate::get_distance_matrix;

pub fn time_between_three_nodes(i1: NodeIndex, i2: NodeIndex, i3: NodeIndex) -> f32{
    let dist = get_distance_matrix();
    
    (if i1 == i2 {0} else {dist.get_edge_weight(i1, i2).unwrap().travel_time}+
        if i2 == i3 {0} else {dist.get_edge_weight(i2, i3).unwrap().travel_time}) as f32
}

pub fn time_between_two_nodes(i1: NodeIndex, i2: NodeIndex) -> f32{
    let dist = get_distance_matrix();

    (if i1 == i2 {0} else {dist.get_edge_weight(i1, i2).unwrap().travel_time}) as f32
}
