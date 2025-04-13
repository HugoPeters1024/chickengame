use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{GameState, assets::ImageAssets};

#[derive(Component)]
#[require(InheritedVisibility, Transform)]
pub struct Car;

pub struct CarPlugin;

impl Plugin for CarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, on_spawn.run_if(in_state(GameState::Running)));
        app.add_systems(Update, cars_drive);
    }
}

fn on_spawn(mut commands: Commands, q: Query<Entity, Added<Car>>, assets: Res<ImageAssets>) {
    for entity in q.iter() {
        commands
            .entity(entity)
            .insert((Collider::cuboid(1.3, 0.9, 0.75), RigidBody::Fixed))
            .with_children(|parent| {
                parent.spawn((SceneRoot(assets.car.clone()),));

                for z in [-0.5, 0.5] {
                    parent.spawn((
                        Transform::from_translation(Vec3::new(1.4, 0.5, z))
                            .looking_at(Vec3::new(2.3, 0.3, z), Vec3::Y),
                        SpotLight {
                            shadows_enabled: true,
                            range: 4.0,
                            intensity: 1000.0 * 1000.0 * 0.1,
                            color: Color::linear_rgb(1.0, 0.0, 0.0),
                            ..default()
                        },
                    ));
                }
            });
    }
}

fn cars_drive(mut q: Query<&mut Transform, With<Car>>) {
    for mut t in q.iter_mut() {
        t.translation.x += 0.002;
    }
}
