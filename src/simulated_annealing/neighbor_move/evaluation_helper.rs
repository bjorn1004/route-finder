use std::cmp::{max, min};
use crate::get_distance_matrix;
use crate::resource::{Time, FULL_DAY};
use petgraph::matrix_graph::NodeIndex;

pub fn time_between_three_nodes(i1: NodeIndex, i2: NodeIndex, i3: NodeIndex) -> Time {
    let dist = get_distance_matrix();

    (if i1 == i2 {
        0
    } else {
        dist.get_edge_weight(i1, i2).unwrap().travel_time
    } + if i2 == i3 {
        0
    } else {
        dist.get_edge_weight(i2, i3).unwrap().travel_time
    })
}

pub fn time_between_two_nodes(i1: NodeIndex, i2: NodeIndex) -> Time {
    let dist = get_distance_matrix();

    if i1 == i2 {
        0
    } else {
        dist.get_edge_weight(i1, i2).unwrap().travel_time
    }
}

/// calculates the time overflow and the time overflow lessened.
///
/// (time_overflow, time_overflow_lessened)
pub fn calculate_time_overflow(time_difference: Time, total_day_time: Time) -> (Time, Time) {
    // first calculates how much time both routes + new time takes.
    // that time minus the maximum time per day is the time overflow.
    // The max is here to turn any negative value into a 0.
    let time_overflow = max(time_difference + total_day_time - FULL_DAY, 0);

    // to get by how much we lessened the overflow value, we calculate 2 things.
    // time_overflow before adding time_difference
    // -time_differnce. This value represents how much overflow could be lessened if we were waay over limits.
    // The lowest value of these 2 is how much the difference lessened the overflow.
    let time_overflow_lessened = min(total_day_time - FULL_DAY, -time_difference);
    (time_overflow, time_overflow_lessened)
}

/// Calculates the capacity overflow and the capacity overflow lessened.
///
/// (capacity_overflow, capacity_overflow_lessened)
///
/// For explanations, see calaculate_time_overflow. This code loks a lot like that
pub fn calculate_capacity_overflow(capacity_difference: i32, route_capacity: i32) -> (i32, i32) {
    let capacity_overflow = max(capacity_difference + route_capacity - 100_000, 0);

    let capacity_overflow_lessened = min(route_capacity - 100_000, -capacity_difference);
    (capacity_overflow, capacity_overflow_lessened)

}
