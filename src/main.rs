use std::{error::Error, fs::File, io::Write};

use petgraph::dot::Dot;

use crate::parser::{parse_distance_matrix, parse_orderfile};

mod parser;
mod resource;

fn main() -> Result<(), Box<dyn Error>> {
    let order_vec = parse_orderfile()?;
    println!("{:?}", order_vec);

    let distance_matrix = parse_distance_matrix()?;
    let mut dot_file = File::create("dotfile.dot")?;
    // Don't actually try to use dot on this file, it will break your PC
    dot_file.write_all(Dot::new(&distance_matrix).to_string().as_bytes())?;

    Ok(())
}
