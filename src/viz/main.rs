use bevy::dev_tools::fps_overlay::FpsOverlayPlugin;
use bevy::prelude::*;
use petgraph::dot::{Config, Dot};

use std::path::Path;

use llvm_viz::types;

#[derive(Component)]
struct Player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(FpsOverlayPlugin::default())
        .add_systems(Startup, (setup_player, setup_camera))
        .add_systems(Update, move_camera)
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
    let rect_width = 100.0; // Width of each rectangle
    let rect_height = 30.0; // Height of each rectangle
    let spacing_x = 50.0; // Horizontal spacing between rectangles
    let spacing_y = 20.0; // Vertical spacing between rows

    // Calculate the optimal number of columns
    // We'll aim for a roughly square layout
    let num_columns = (num_nodes as f32).sqrt().ceil() as usize;
    let box_width = rect_width + spacing_x;
    let box_height = rect_height + spacing_y;

    let total_width = (rect_width + spacing_x) * (num_columns as f32) - spacing_x;
    let total_height = (rect_height + spacing_y) * (num_columns as f32) - spacing_y;

    for (i, node) in g.raw_nodes().iter().enumerate() {
        commands
            .spawn((
                Player,
                Mesh2d(meshes.add(Rectangle::new(100., 30.))),
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
    commands.spawn((Camera2d, Camera::default()));
}

fn move_camera(
    mut camera: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
    kb_input: Res<ButtonInput<KeyCode>>,
) {
    let Ok(mut camera) = camera.get_single_mut() else {
        return;
    };

    let mut direction = Vec2::ZERO;

    if kb_input.pressed(KeyCode::KeyW) {
        direction.y += 1.;
    }

    if kb_input.pressed(KeyCode::KeyS) {
        direction.y -= 1.;
    }

    if kb_input.pressed(KeyCode::KeyA) {
        direction.x -= 1.;
    }

    if kb_input.pressed(KeyCode::KeyD) {
        direction.x += 1.;
    }

    let move_delta = direction.normalize_or_zero() * 10.;
    camera.translation += move_delta.extend(0.);
}
