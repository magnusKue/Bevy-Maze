use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_player);
    }
}

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // player size
    let player_size = Vec3::new(0.3, 0.8, 0.3); 

    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(player_size.x, player_size.y, player_size.z)),
        material: materials.add(Color::RED),
        transform: Transform::from_xyz(1.0, 1.0, 4.0),
        ..default()
    })
    .insert(Collider::cuboid(0.5 * player_size.x, 0.5 * player_size.y, 0.5 * player_size.z))
    .insert(Name::new("Player".to_string()))
    .insert(RigidBody::Dynamic)
    .insert(GravityScale(0.1))
    .insert(LockedAxes::ROTATION_LOCKED_X | LockedAxes::ROTATION_LOCKED_Z)
    .insert(Ccd::enabled());
}