
use std::fmt::Display;

use super::route::Route;
use rand::Rng;
use rand::distr::{Distribution, StandardUniform};
pub struct Day {
    morning: Route,
    afternoon: Route,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
pub enum TimeOfDay{
    Morning,
    Afternoon
}

impl Display for TimeOfDay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let show = match self {
           Self::Morning => "Morning",
           Self::Afternoon => "Afternoon"
        };
        write!(f, "{show}")
    }
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
