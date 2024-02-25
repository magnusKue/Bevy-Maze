use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
pub struct MazePlugin;

impl Plugin for MazePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_map);
    }
}

fn spawn_map(
    mut commands: Commands,
    ass: Res<AssetServer>,
) {

    let map = [
        // [2,2,2],
        // [2,1,2],
        // [2,2,2],
        [2,1,1,1,1,2],
        [1,0,0,0,0,1],
        [1,0,0,0,0,1],
        [2,1,1,1,1,2],
    ];
    
    let corner = ass.load("maze_corner.glb#Scene0"); // 2
    let wall = ass.load("maze_wall.glb#Scene0");   // 1
    let floor = ass.load("maze_floor.glb#Scene0");  // 0
    let tunnel = ass.load("maze_tunnel.glb#Scene0"); // 3

    for (y, column) in map.iter().enumerate() {
        for (x, number) in column.iter().enumerate() {
            commands.spawn(SceneBundle {
                scene: match number {
                    0 => floor.clone(),
                    1 => wall.clone(),
                    2 => corner.clone(),
                    3 => tunnel.clone(),
                    _ => corner.clone(),
                },
                transform: Transform::from_xyz(
                    (x as f32)*2.0, 
                    0.0 , 
                    (y as f32)*2.0
                ),
                ..Default::default()
            })
            // .insert(Collider::cylinder(0.55, 18.0))
            // .insert(RigidBody::Fixed)
            .insert(Name::new("map_part".to_string()))
            .with_children(|children| {
                children.spawn(Collider::cuboid(1.0, 0.08, 1.0))
                    // Position the collider relative to the rigid-body.
                    .insert(TransformBundle::from(Transform::from_xyz(0.0,-0.08,0.0)));
            });
        }
    }

    // 'Material0', 'Mesh0', 'Mesh0/Primitive0', 'Mesh1', 'Mesh1/Primitive0', 'Mesh2', 'Mesh2/Primitive0', 'Node0', 'Node1', 'Node2', 'Scene0', 'Texture0'
}