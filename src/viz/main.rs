use std::path::Path;

extern crate piston_window;
use petgraph::dot::{Config, Dot};
use piston_window::*;

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

    let mut window: PistonWindow = WindowSettings::new("Hello Piston!", [640, 480])
        .exit_on_esc(true)
        .build()
        .unwrap();

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, _device| {
            clear([1.0; 4], g);

            for node in nodes.clone() {
                // todo: Also display node's name
                rectangle([0.5, 0.7, 0.0, 1.0], node.coords, c.transform, g);
            }
        });
    }
}
