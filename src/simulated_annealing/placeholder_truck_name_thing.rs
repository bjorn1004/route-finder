use crate::datastructures::compact_linked_vector::CompactLinkedVector;
use crate::resource::MatrixID;
use super::route::Route;

pub struct PlaceholderTruckNameThing{
    // A list of 5 lists, where each list contains 2 CompactLinkedVectors for the 2 routes.
    pub routes: Vec<Vec<Route>>,
}


impl PlaceholderTruckNameThing{
    fn new() -> Self{
        let route_for_one_day = vec![
            Route::new(),
            Route::new(),
        ];
        let routes = vec!{
            route_for_one_day.clone(),
            route_for_one_day.clone(),
            route_for_one_day.clone(),
            route_for_one_day.clone(),
            route_for_one_day.clone(),
        };
        PlaceholderTruckNameThing{
            routes
        }
    }

    pub fn get_mut_route_on_day(&mut self, day:usize) -> &mut Vec<Route>{
        &mut self.routes[day-1]
    }
}