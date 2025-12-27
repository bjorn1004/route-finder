use std::{fmt::Display, str::FromStr};

use petgraph::{matrix_graph::DiMatrix, prelude::*};

#[derive(Debug, Clone)]
pub struct Company {
    pub order: u16,
    pub place: String,
    pub frequency: Frequency,
    pub container_count: u8,
    pub container_volume: u16,
    pub emptying_time: Time,
    pub matrix_id: MatrixID,
    pub x_coordinate: u32,
    pub y_coordinate: u32, // maybe turn this into a nalgebra vector if we need it
    pub total_container_volume: u32,
    pub penalty: Time,
}
pub type MatrixID = NodeIndex<u16>;
/// time in centiseconds
pub type Time = i32;
/// 60 * 100 centiseconds
pub const MINUTE:Time = 60*100;
/// 30 * 60 * 100 centiseconds
pub const HALF_HOUR:Time = 30*60*100;
/// 12 * 60 * 60 * 100 centiseconds;
pub const FULL_DAY:Time = 12*60*60*100;

#[derive(Debug, Clone, Copy)]
pub enum Frequency {
    None = 0, // This is only for the dropoff location
    Once = 1,
    Twice = 2,
    Thrice = 3,
    FourTimes = 4,
}

impl FromStr for Frequency {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "1PWK" => Ok(Frequency::Once),
            "2PWK" => Ok(Frequency::Twice),
            "3PWK" => Ok(Frequency::Thrice),
            "4PWK" => Ok(Frequency::FourTimes),
            _ => Err(format!("Invalid frequency: {}", s)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Distance {
    pub absolute_distance: u16,
    pub travel_time: Time,
}

impl Display for Distance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Distance: {}, Travel time: {}",
            self.absolute_distance, self.travel_time
        )
    }
}

pub type DistanceMatrix = DiMatrix<MatrixID, Distance>;
