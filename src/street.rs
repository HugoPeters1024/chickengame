use std::f32::consts::PI;

use bevy::{
    math::Affine2,
    prelude::*,
    utils::{HashMap, HashSet},
};
use bevy_rapier3d::prelude::*;

use crate::{GameState, Player, assets::ImageAssets, lantern::Lantern};

pub struct StreetPlugin;

const GRID_WIDTH: f32 = 2.0;
const GRID_HEIGHT: f32 = 2.0;
const SPAWN_DISTANCE_TILES: i32 = 8;

impl Plugin for StreetPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StreetTiles>();
        app.add_systems(
            Update,
            ensure_tiles_spawned.run_if(in_state(GameState::Running)),
        );
    }
}

#[derive(Resource, Default)]
struct StreetTiles(HashMap<IVec2, Entity>);

fn ensure_tiles_spawned(
    mut commands: Commands,
    mut street_tiles: ResMut<StreetTiles>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    image_assets: Res<ImageAssets>,
    q: Query<&GlobalTransform, With<Player>>,
) {
    let player = q.single();
    let grid_loc = IVec2::new(
        (player.translation().x / GRID_WIDTH) as i32,
        (player.translation().z / GRID_HEIGHT) as i32,
    );

    let mut despawn: HashSet<Entity> = street_tiles.0.values().cloned().collect();

    let mut spawn_grid_item = |x: i32, y: i32, commands: &mut Commands| -> Entity {
        if y.abs() <= 1 {
            commands
                .spawn((
                    Mesh3d(meshes.add(Cuboid::new(GRID_WIDTH, 0.1, GRID_HEIGHT))),
                    MeshMaterial3d(materials.add(StandardMaterial {
                        base_color_texture: Some(image_assets.road.clone()),
                        uv_transform: Affine2::from_scale_angle_translation(
                            Vec2::new(1.0, 1.0),
                            PI / 2.0,
                            Vec2::ZERO,
                        ),
                        ..default()
                    })),
                    Transform::from_xyz(x as f32 * GRID_WIDTH, -0.1, y as f32 * GRID_HEIGHT),
                    RigidBody::Fixed,
                    Collider::cuboid(GRID_WIDTH / 2.0, 0.05, GRID_HEIGHT / 2.0),
                ))
                .with_children(|parent| {
                    if x % 4 == 0 && y != 0 {
                        parent.spawn((
                            Lantern,
                            Transform::from_rotation(Quat::from_rotation_y(
                                -PI / 2.0 * if y > 0 { -1.0 } else { 1.0 },
                            ))
                            .with_translation(Vec3::new(
                                0.0,
                                0.0,
                                1.2 * if y > 0 { 1.0 } else { -1.0 },
                            )),
                        ));
                    }
                })
                .id()
        } else if y.abs() <= 2 {
            commands
                .spawn((
                    Mesh3d(meshes.add(Cuboid::new(GRID_WIDTH, 0.1, GRID_HEIGHT))),
                    MeshMaterial3d(materials.add(StandardMaterial {
                        base_color_texture: Some(image_assets.tiles.clone()),
                        uv_transform: Affine2::from_scale_angle_translation(
                            Vec2::new(1.0, 1.0),
                            PI / 2.0,
                            Vec2::ZERO,
                        ),
                        ..default()
                    })),
                    Transform::from_xyz(x as f32 * GRID_WIDTH, -0.1, y as f32 * GRID_HEIGHT),
                    RigidBody::Fixed,
                    Collider::cuboid(GRID_WIDTH / 2.0, 0.05, GRID_HEIGHT / 2.0),
                ))
                .id()
        } else {
            commands
                .spawn((
                    Mesh3d(meshes.add(Cuboid::new(GRID_WIDTH, 0.1, GRID_HEIGHT))),
                    MeshMaterial3d(materials.add(StandardMaterial {
                        base_color_texture: Some(image_assets.grass.clone()),
                        uv_transform: Affine2::from_scale_angle_translation(
                            Vec2::new(1.0, 1.0),
                            PI / 2.0,
                            Vec2::ZERO,
                        ),
                        ..default()
                    })),
                    Transform::from_xyz(x as f32 * GRID_WIDTH, -0.1, y as f32 * GRID_HEIGHT),
                    RigidBody::Fixed,
                    Collider::cuboid(GRID_WIDTH / 2.0, 0.05, GRID_HEIGHT / 2.0),
                ))
                .id()
        }
    };

    for dy in -SPAWN_DISTANCE_TILES..=SPAWN_DISTANCE_TILES {
        for dx in -SPAWN_DISTANCE_TILES..=SPAWN_DISTANCE_TILES {
            let x = grid_loc.x + dx;
            let y = grid_loc.y + dy;
            let loc = IVec2::new(x, y);
            if !street_tiles.0.contains_key(&loc) {
                street_tiles
                    .0
                    .insert(loc, spawn_grid_item(loc.x, loc.y, &mut commands));
            } else {
                despawn.remove(street_tiles.0.get(&loc).unwrap());
            }
            match y {
                -1 | 0 | 1 => {}
                2 | -2 => {
                    if !street_tiles.0.contains_key(&loc) {
                        street_tiles
                            .0
                            .insert(loc, spawn_grid_item(loc.x, loc.y, &mut commands));
                    } else {
                        despawn.remove(street_tiles.0.get(&loc).unwrap());
                    }
                }
                _ => {}
            }
        }
    }

    for entity in &despawn {
        commands.entity(*entity).despawn_recursive();
    }

    street_tiles.0.retain(|_, v| !despawn.contains(v));
}
