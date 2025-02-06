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

    // TODO: Add name
    for (i, node) in g.raw_nodes().iter().enumerate() {
        commands
            .spawn((
                Player,
                Mesh2d(meshes.add(Rectangle::new(100., 30.))),
                MeshMaterial2d(materials.add(Color::hsv(
                    360. * (i as f32) / g.raw_nodes().iter().count() as f32,
                    1.,
                    1.,
                ))),
                Transform::from_xyz(150. * i as f32, 0., 2.),
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

    let move_delta = direction.normalize_or_zero();
    camera.translation += move_delta.extend(0.);
}
