use bevy::prelude::*;
use bevy_mod_picking::*;

mod pieces;
use pieces::*;
mod board;
use board::*;

// From: https://caballerocoll.com/blog/bevy-chess-tutorial/

fn main() {
    App::build().
    // Add 4x MSAA
    add_resource(Msaa {samples: 4}).
    add_resource(WindowDescriptor {
      title: "Rusty Chess".to_string(),
      width: 1600.,
      height: 1000.,
      ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .add_plugin(BoardPlugin)
    .add_plugin(PiecesPlugin)
    .add_plugin(PickingPlugin)
    // .add_plugin(DebugPickingPlugin)
    .add_startup_system(setup.system())
    .run();
}

fn setup(commands: &mut Commands) {
    commands
        // Camera
        .spawn(Camera3dBundle {
            transform: Transform::from_matrix(Mat4::from_rotation_translation(
                Quat::from_xyzw(-0.3, -0.5, -0.3, 0.5).normalize(),
                Vec3::new(-5.0, 15.0, 4.0),
            )),
            ..Default::default()
        })
        .with(PickSource::default())
        // Light
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
            ..Default::default()
        });
}
