use std::cmp::max;
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

/// calculates the time overflow delta.
///
/// (time_overflow_delta)
pub fn calculate_time_overflow(time_difference: Time, total_day_time: Time) -> Time {
    // first calculates how much time both routes + new time takes.
    // that time minus the maximum time per day is the time overflow.
    // The max is here to turn any negative value into a 0.
    let old_overflow = max(total_day_time - FULL_DAY, 0);
    let new_overflow = max(time_difference + total_day_time - FULL_DAY, 0);

    new_overflow - old_overflow
}

/// Calculates the capacity overflow delta.
///
/// (capacity_overflow_delta)
///
/// For explanations, see calaculate_time_overflow. This code loks a lot like that
pub fn calculate_capacity_overflow(capacity_difference: i32, route_capacity: i32) -> i32 {
    let old_overflow = max(route_capacity - 100_000,0);
    let new_overflow = max(route_capacity + capacity_difference - 100_000, 0);

    new_overflow - old_overflow
}
