use std::path::Path;

use petgraph::dot::{Config, Dot};

use llvm_viz::types;

#[derive(Debug, Clone)]
struct Rectangle {
    name: String,
    coords: [f64; 4],
}

fn main() {
    let g = types::graph_from_json(Path::new("omega_tree.json")).expect("");
    println!("{:?}", Dot::with_config(&g, &[Config::EdgeNoLabel]));

    let nodes: Vec<_> = g
        .raw_nodes()
        .iter()
        .enumerate()
        .map(|(i, n)| Rectangle {
            name: n.weight.name.clone(),
            coords: [i as f64 * 150.0 + 50 as f64, 100.0, 100.0, 100.0],
        })
        .collect();
}
