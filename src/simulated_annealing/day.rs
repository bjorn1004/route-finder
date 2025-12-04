use rand::distr::{Distribution, StandardUniform};
use rand::Rng;
use crate::simulated_annealing::week::DayEnum;
use super::route::Route;
pub struct Day{
    morning: Route,
    afternoon: Route
}

impl Day {
    pub fn new() -> Self{
        Day{
            morning: Route::new(),
            afternoon: Route::new()
        }
    }
    pub fn get_random<R: Rng + ?Sized>(&self, rng: &mut R) -> &Route{
        let a:bool = rng.random_bool(0.5);
        match a {
            True => &self.morning,
            False => &self.afternoon,
        }
    }
}