use crate::simulated_annealing::placeholder_truck_name_thing::PlaceholderTruckNameThing;
use crate::datastructures::linked_vectors::{LinkedVector, NodeIndex};
use rand::{Rng};
use rand::rngs::SmallRng;
use crate::resource::MatrixID;

pub trait TransactionNeighborThing{
    // This function would get random values from the routes, and store important information in
    // the struct that implements this trait.
    fn new(_: &Vec<PlaceholderTruckNameThing>, rng: &mut SmallRng) -> Self where Self: Sized;
    // this would return the difference in volume or time
    // (not sure how to implement this yet)
    fn evaluate(&self);
    // this would perform the thing on the schedules.
    fn execute(&self, _: &mut Vec<PlaceholderTruckNameThing>);
}

pub struct Swap2RandomValuesInSameRoute {
    truck: usize,
    day: usize,
    route_of_day: usize,
    index1: NodeIndex,
    index2: NodeIndex,
    matrix_id1: MatrixID,
    matrix_id2: MatrixID,

}

/// This tihng will change nothing, it is purely here to find what variables we would need in the trait above.
impl TransactionNeighborThing for Swap2RandomValuesInSameRoute {

    fn new(trucks: &Vec<PlaceholderTruckNameThing>, rng: &mut SmallRng) -> Self{
        let (truck, day, route_of_day): (u8, u8, u8) = rng.random();
        let truck:usize = truck as usize%2;
        let day:usize = day as usize%5;
        let route_of_day:usize = route_of_day as usize%2;

        let route = &trucks[truck].routes[day][route_of_day];
        let (index1, matrix_id1) = route.linked_vector.get_random(rng).unwrap();
        let (index2, matrix_id2) = route.linked_vector.get_random(rng).unwrap();
        Swap2RandomValuesInSameRoute{
            truck,
            day,
            route_of_day,
            index1,
            index2,
            matrix_id1: matrix_id1.clone(),
            matrix_id2: matrix_id2.clone(),
        }
    }

    fn evaluate(&self) {
        todo!()
    }

    fn execute(&self, trucks: &mut Vec<PlaceholderTruckNameThing>) {
        let route = &mut trucks[self.truck].routes[self.day][self.route_of_day].linked_vector;
        route.set_value_at_index(self.index1, self.matrix_id2);
        route.set_value_at_index(self.index2, self.matrix_id1);
    }
}

pub struct Swap2RandomValuesInSameRouteYAAY {
    truck: usize,
    day: usize,
    route_of_day: usize,
    index1: NodeIndex,
    index2: NodeIndex,
    matrix_id1: MatrixID,
    matrix_id2: MatrixID,

}
/// This tihng will change nothing, it is purely here to find what variables we would need in the trait above.
impl TransactionNeighborThing for Swap2RandomValuesInSameRouteYAAY {

    fn new(trucks: &Vec<PlaceholderTruckNameThing>, rng: &mut SmallRng) -> Self {
        let (truck, day, route_of_day): (u8, u8, u8) = rng.random();
        let truck:usize = truck as usize%2;
        let day:usize = day as usize%5;
        let route_of_day:usize = route_of_day as usize%2;

        let route = &trucks[truck].routes[day][route_of_day];
        let (index1, matrix_id1) = route.linked_vector.get_random(rng).unwrap();
        let (index2, matrix_id2) = route.linked_vector.get_random(rng).unwrap();
        Swap2RandomValuesInSameRouteYAAY{
            truck,
            day,
            route_of_day,
            index1,
            index2,
            matrix_id1: matrix_id1.clone(),
            matrix_id2: matrix_id2.clone(),
        }
    }

    fn evaluate(&self) {
        todo!()
    }

    fn execute(&self, trucks: &mut Vec<PlaceholderTruckNameThing>) {
        let route = &mut trucks[self.truck].routes[self.day][self.route_of_day].linked_vector;
        route.set_value_at_index(self.index1, self.matrix_id2);
        route.set_value_at_index(self.index2, self.matrix_id1);
    }
}
