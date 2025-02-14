use bevy::dev_tools::fps_overlay::FpsOverlayPlugin;
use bevy::prelude::*;
use bevy_pancam::{PanCam, PanCamPlugin};
use petgraph::dot::{Config, Dot};

use std::path::Path;

use llvm_viz::types;

#[derive(Component)]
struct Player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(FpsOverlayPlugin::default())
        .add_plugins(PanCamPlugin)
        .add_systems(Startup, (setup_player, setup_camera))
        .run();
}

fn setup_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let g = types::graph_from_json(Path::new("omega_tree.json")).expect("");
    println!("{:?}", Dot::with_config(&g, &[Config::EdgeNoLabel]));

    let num_nodes = g.raw_nodes().len();
    const RECT_WIDTH: f32 = 300.0; // Width of each rectangle
    const RECT_HEIGHT: f32 = 30.0; // Height of each rectangle
    const SPACING_X: f32 = 50.0; // Horizontal spacing between rectangles
    const SPACING_Y: f32 = 20.0; // Vertical spacing between rows

    // Calculate the optimal number of columns
    // We'll aim for a roughly square layout
    let num_columns = (num_nodes as f32).sqrt().ceil() as usize;
    let box_width = RECT_WIDTH + SPACING_X;
    let box_height = RECT_HEIGHT + SPACING_Y;

    let total_width = (RECT_WIDTH + SPACING_X) * (num_columns as f32) - SPACING_X;
    let total_height = (RECT_HEIGHT + SPACING_Y) * (num_columns as f32) - SPACING_Y;

    for (i, node) in g.raw_nodes().iter().enumerate() {
        commands
            .spawn((
                Player,
                Mesh2d(meshes.add(Rectangle::new(RECT_WIDTH, RECT_HEIGHT))),
                MeshMaterial2d(materials.add(Color::hsv(
                    360. * (i as f32) / num_nodes as f32,
                    1.,
                    1.,
                ))),
                Transform::from_xyz(
                    (-total_width / 2.0) + (i % num_columns) as f32 * box_width,
                    (total_height / 2.0) - ((i / num_columns) as f32 * box_height),
                    2.,
                ),
            ))
            .with_child(Text2d::new(node.weight.name.clone()));
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2d, PanCam::default()));
}
