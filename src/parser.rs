use std::error::Error;

use petgraph::{matrix_graph::MatrixGraph, prelude::GraphMap};

use crate::resource::{Company, Distance, DistanceMatrix};

// Small helper function for getting columns
fn get_next(
    cols: &mut std::str::Split<'_, char>,
    field_name: &str,
) -> Result<String, Box<dyn Error + 'static>> {
    Ok(cols
        .next()
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Error column missing {}", field_name))?)
}

pub fn parse_orderfile() -> Result<Vec<Company>, Box<dyn Error>> {
    let orderfile = include_str!("../data/Orderbestand.txt");

    // Split in lines, skip headers
    Ok(orderfile
        .lines()
        .skip(1)
        .map(|line| -> Result<Company, Box<dyn Error>> {
            let mut columns = line.split(';');

            Ok(Company {
                order: get_next(&mut columns, "Order")?.parse()?,
                place: String::from(get_next(&mut columns, "Plaats")?.trim()),
                frequency: get_next(&mut columns, "Frequentie")?.parse()?,
                container_count: get_next(&mut columns, "AantContainers")?.parse()?,
                container_volume: get_next(&mut columns, "VolumePerContainer")?.parse()?,
                emptying_time: get_next(&mut columns, "LedigingsDuurMinuten")?.parse()?,
                matrix_id: get_next(&mut columns, "MatrixID")?.parse()?,
                x_coordinate: get_next(&mut columns, "XCoordinaat")?.parse()?,
                y_coordinate: get_next(&mut columns, "YCoordinaat")?.parse()?,
            })
        })
        .collect::<Result<Vec<Company>, Box<dyn Error>>>()?)
}

pub fn parse_distance_matrix() -> Result<DistanceMatrix, Box<dyn Error>> {
    let distance_matrix_file = include_str!("../data/AfstandenMatrix.txt");

    distance_matrix_file.lines().skip(1).try_fold(
        MatrixGraph::new(),
        |mut graph, line| -> Result<DistanceMatrix, Box<dyn Error>> {
            let mut colunms = line.split(';');

            let node_a: u16 = get_next(&mut colunms, "MatrixID1")?.parse()?;
            let node_b: u16 = get_next(&mut colunms, "MatrixID2")?.parse()?;
            let distance = Distance {
                absolute_distance: get_next(&mut colunms, "Afstand")?.parse()?,
                travel_time: get_next(&mut colunms, "Rijtijd")?.parse()?,
            };

            graph.add_edge(node_a.into(), node_b.into(), distance);

            Ok(graph)
        },
    )
}
