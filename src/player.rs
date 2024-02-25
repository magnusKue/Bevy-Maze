use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PrimaryWindow};
use bevy_rapier3d::prelude::*;
use bevy::input::mouse::MouseMotion;
// camera
use bevy_atmosphere::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct CameraMarker; 

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_player)
            .add_systems(Update, move_player);
    }
}

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    // player size
    let player_size = Vec3::new(0.3, 0.8, 0.3); 

    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(player_size.x, player_size.y, player_size.z)),
        material: materials.add(Color::RED),
        transform: Transform::from_xyz(1.0, 1.0, 4.0),
        ..default()
    })
    .insert(Name::new("Player".to_string()))
    .insert(Player)
    
    .insert(Collider::cuboid(0.5 * player_size.x, 0.5 * player_size.y, 0.5 * player_size.z))
    .insert((RigidBody::KinematicVelocityBased, KinematicCharacterController::default()))
    .insert(GravityScale(0.1))
    .insert(LockedAxes::ROTATION_LOCKED_X | LockedAxes::ROTATION_LOCKED_Z)
    .insert(Ccd::enabled())

    .with_children(| children | {
        children.spawn((CameraMarker, Camera3dBundle::default(), AtmosphereCamera::default()));
    });


    // lock and hide cursor

    let mut primary_window = q_windows.single_mut();

    // if you want to use the cursor, but not let it leave the window,
    // use `Confined` mode:
    primary_window.cursor.grab_mode = CursorGrabMode::Confined;

    // for a game that doesn't use the cursor (like a shooter):
    // use `Locked` mode to keep the cursor in one place
    primary_window.cursor.grab_mode = CursorGrabMode::Locked;

    // also hide the cursor
    primary_window.cursor.visible = false;

}

pub fn move_player(
    input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut KinematicCharacterController, &mut Transform), With<Player>>,
    mut camera_query: Query<&mut Transform, (With<CameraMarker>, Without<Player>)>,
    mut motion_evr: EventReader<MouseMotion>,
) {
    for (mut controller, mut transform) in player_query.iter_mut() {
        let mut cam_transform = camera_query.get_single_mut().unwrap();

        // mouse movement
        let rotationspeed = 0.001;
        for ev in motion_evr.read() {
            transform.rotate_y(-ev.delta.x * rotationspeed);
            cam_transform.rotate_local_x(-ev.delta.y * rotationspeed);
        }
        motion_evr.clear();

        // Keyboard input
        let movespeed = 0.04;
        let mut offset = Vec3::new(0.0,-0.1,0.0);
        if input.pressed(KeyCode::KeyW) {
            offset += Vec3::new(-transform.local_z().x, 0.0, -transform.local_z().z).normalize_or_zero();
        }
        else if input.pressed(KeyCode::KeyS) {
            offset += Vec3::new(transform.local_z().x, 0.0, transform.local_z().z).normalize_or_zero();
        }

        if input.pressed(KeyCode::KeyA) {
            offset += Vec3::new(-transform.local_x().x, 0.0, -transform.local_x().z).normalize_or_zero();
        }
        else if input.pressed(KeyCode::KeyD) {
            offset += Vec3::new(transform.local_x().x, 0.0, transform.local_x().z).normalize_or_zero();
        }
        controller.translation = Some(offset.normalize() * movespeed);
    }
}