use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{GameState, assets::ImageAssets};

#[derive(Component)]
#[require(InheritedVisibility, Transform)]
pub struct Lantern;

#[derive(Default)]
pub struct LanternPlugin;

impl Plugin for LanternPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, on_spawn.run_if(in_state(GameState::Running)));
    }
}

fn on_spawn(mut commands: Commands, q: Query<Entity, Added<Lantern>>, assets: Res<ImageAssets>) {
    for entity in q.iter() {
        commands
            .entity(entity)
            .insert((Collider::cuboid(0.3, 3.0, 0.3), RigidBody::Fixed))
            .with_children(|parent| {
                parent.spawn((
                    SceneRoot(assets.lantern.clone()),
                    Transform::from_scale(Vec3::splat(0.15)),
                ));

                parent.spawn((
                    Transform::from_translation(Vec3::new(1.0, 2.4, 0.0))
                        .looking_at(Vec3::new(1.0, 0.0, 0.0), Vec3::Y),
                    SpotLight {
                        shadows_enabled: true,
                        range: 4.0,
                        intensity: 1000.0 * 1000.0 * 0.1,
                        ..default()
                    },
                    //PointLight {
                    //    shadows_enabled: true,
                    //    range: 4.0,
                    //    intensity: 1000.0 * 1000.0 * 0.1,
                    //    ..default()
                    //},
                ));
            });
    }
}
