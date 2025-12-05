use super::route::Route;
use crate::simulated_annealing::week::DayEnum;
use rand::Rng;
use rand::distr::{Distribution, StandardUniform};
pub struct Day {
    morning: Route,
    afternoon: Route,
}

pub enum TimeOfDay{
    Morning,
    Afternoon
}

// This makes it easier to get a random day
impl Distribution<TimeOfDay> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> TimeOfDay {
        match rng.random_range(0..2) {
            0 => TimeOfDay::Morning,
            _ => TimeOfDay::Afternoon,
        }
    }
}
impl Day {
    pub fn new() -> Self {
        Day {
            morning: Route::new(),
            afternoon: Route::new(),
        }
    }
    pub fn get_random<R: Rng + ?Sized>(&self, rng: &mut R) -> (&Route, TimeOfDay) {
        match rng.random() {
            TimeOfDay::Morning => (&self.morning, TimeOfDay::Morning),
            TimeOfDay::Afternoon => (&self.afternoon, TimeOfDay::Morning)
        }
    }
}
