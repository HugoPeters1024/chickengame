use std::f32::consts::PI;

use bevy::{math::Affine2, prelude::*, utils::HashMap};

use crate::{GameState, Player, assets::ImageAssets};

pub struct StreetPlugin;

const GRID_WIDTH: f32 = 2.0;
const GRID_HEIGHT: f32 = 2.0;
const SPAWN_DISTANCE_TILES: i32 = 5;

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

    for dy in -3..=3 {
        for dx in -3..=3 {
            let x = grid_loc.x + dx;
            let y = grid_loc.y + dy;
            if y > 1 || y < -1 { continue };
            if !street_tiles.0.contains_key(&IVec2::new(x, y)) {
                street_tiles.0.insert(
                    IVec2::new(x, y),
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
                        ))
                        .id(),
                );
            }
        }
    }
}
