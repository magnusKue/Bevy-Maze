use bevy::prelude::*;
use bevy_atmosphere::prelude::*;
use bevy_basic_camera::{CameraController, CameraControllerPlugin};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;

pub mod maze;
pub mod player;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins, 
            AtmospherePlugin,
            CameraControllerPlugin
        ))

        .add_plugins(maze::MazePlugin)
        .add_plugins(player::PlayerPlugin)

        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin{enabled:true, ..default()})

        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, (
            setup,
        ))
        .run();
}

/// set up a simple 3D scene
pub fn setup(
    mut commands: Commands,
) {
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            intensity: 1_000_000.0,
            ..default()
        },
        transform: Transform::from_xyz(-1.0, 8.7, -1.0),
        ..default()
    });
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    })
    .insert(AtmosphereCamera::default())
    .insert(CameraController::default());
}