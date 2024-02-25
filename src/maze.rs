use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Component)]
pub struct Maze; 
pub struct MazePlugin;

impl Plugin for MazePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_map);
    }
}

pub fn spawn_map(
    mut commands: Commands,
    ass: Res<AssetServer>,
) {
    let _maze = commands.spawn((Maze, Name::new("Maze".to_string()))).id();

    let map = [
        [1,1,1,1,1,1,1,1,1,1],
        [1,0,1,0,1,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,1],
        [1,0,0,1,0,0,0,0,0,1],
        [1,1,1,1,1,1,1,1,1,1],
    ];
    
    let wall = ass.load("maze_wall.glb#Scene0");   // 1
    let floor = ass.load("maze_floor.glb#Scene0");  // 0

    for (y, column) in map.iter().enumerate() {
        for (x, number) in column.iter().enumerate() {
            let mesh = match number {
                0 => Ok(floor.clone()),
                1 => Ok(wall.clone()),
                _ => Err(()),
            }.expect("Unknown tiletype");

            let collider_hy = match number {
                0 => Ok(0.08),
                1 => Ok(2.0),
                _ => Err(()),
            }.expect("Unknown tiletype");


            commands.spawn(SceneBundle {
                scene: mesh,
                transform: Transform::from_xyz(
                    (x as f32)*2.0, 
                    0.0 , 
                    (y as f32)*2.0
                ),
                ..Default::default()
            })

            .insert(RigidBody::Fixed)
            .insert(Name::new("map_part".to_string()))
            
            .with_children(|children| {
                children.spawn(Collider::cuboid(1.0, collider_hy, 1.0))
                    // Position the collider relative to the rigid-body.
                    .insert(TransformBundle::from(Transform::from_xyz(0.0,-0.08,0.0)));
            });
        }
    }
}