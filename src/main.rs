use std::{
    cell::OnceCell,
    error::Error,
    fs::File,
    io::Write,
    sync::{OnceLock, RwLock},
};

use petgraph::dot::Dot;

use crate::{
    parser::{parse_distance_matrix, parse_orderfile},
    resource::{Company, DistanceMatrix},
};

mod datastructures;
mod parser;
mod resource;

pub static ORDERS: OnceLock<Vec<Company>> = OnceLock::new();
pub static DISTANCE_MATRIX: OnceLock<DistanceMatrix> = OnceLock::new();

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let instant = std::time::Instant::now();
    let order_vec = parse_orderfile()?;
    ORDERS.get_or_init(|| order_vec);
    let distance_matrix = parse_distance_matrix()?;
    DISTANCE_MATRIX.get_or_init(|| distance_matrix);

    println!("{:?}", ORDERS.get().unwrap());

    // let mut dot_file = File::create("dotfile.dot")?;
    // Don't actually try to use dot on this file, it will break your PC
    // dot_file.write_all(
    // Dot::new(&DISTANCE_MATRIX.get().unwrap())
    // .to_string()
    // .as_bytes(),
    // )?;

    println!("{}", instant.elapsed().as_secs_f64());
    Ok(())
}
