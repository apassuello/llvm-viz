use std::path::Path;

use petgraph::dot::{Config, Dot};
//use petgraph::Graph;

use llvm_viz::types;

fn main() {
    let g = types::graph_from_json(Path::new("omega_tree.json")).expect("");

    println!("{:?}", Dot::with_config(&g, &[Config::EdgeNoLabel]));
}
