use bevy::prelude::*;
use bevy_atmosphere::prelude::*;
use bevy_basic_camera::{CameraController, CameraControllerPlugin};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;

pub mod maze;
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins, 
            AtmospherePlugin,
            CameraControllerPlugin
        ))

        .add_plugins(maze::MazePlugin)

        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())

        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, (
            setup,
            spawn_player
        ))
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
) {
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(-1.6, 8.7, -10.5),
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



fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // player size
    let pls = Vec3::new(0.3, 0.8, 0.3); 

    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(pls.x, pls.y, pls.z)),
        material: materials.add(Color::RED),
        transform: Transform::from_xyz(-5.1, 5.0, -10.0),
        ..default()
    })
    .insert(Collider::cuboid(0.5 * pls.x, 0.5 * pls.y, 0.5 * pls.z))
    .insert(Name::new("Player".to_string()))
    .insert(RigidBody::Dynamic)
    .insert(GravityScale(0.1))
    .insert(LockedAxes::ROTATION_LOCKED_X | LockedAxes::ROTATION_LOCKED_Z)
    .insert(Ccd::enabled());
}