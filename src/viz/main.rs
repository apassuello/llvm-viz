use bevy::prelude::*;
use petgraph::dot::{Config, Dot};

use std::path::Path;

use llvm_viz::types;

#[derive(Debug, Clone, Component)]
struct Rectangle {
    name: String,
    coords: [f64; 4],
}

fn add_rectangles(mut commands: Commands) {
    let g = types::graph_from_json(Path::new("omega_tree.json")).expect("");
    println!("{:?}", Dot::with_config(&g, &[Config::EdgeNoLabel]));

    for r in g
        .raw_nodes()
        .iter()
        .enumerate()
        .map(|(i, n)| Rectangle {
            name: n.weight.name.clone(),
            coords: [i as f64 * 150.0 + 50 as f64, 100.0, 100.0, 100.0],
        })
        .collect::<Vec<_>>()
    {
        commands.spawn(r);
    }
}

fn greet_rectantles(query: Query<&Rectangle>) {
    for rectangle in &query {
        println!("hello {}!", rectangle.name);
    }
}

fn main() {
    App::new()
        .add_systems(Startup, add_rectangles)
        .add_systems(Update, greet_rectantles)
        .run();
}
