use assets::ImageAssets;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_rapier3d::prelude::*;
use car::{Car, CarPlugin};
use lantern::LanternPlugin;
use street::StreetPlugin;

mod assets;
mod car;
mod lantern;
mod street;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum GameState {
    #[default]
    Loading,
    Running,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
//        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins((LanternPlugin, StreetPlugin, CarPlugin))
        .init_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::Running)
                .load_collection::<ImageAssets>(),
        )
        .add_systems(OnEnter(GameState::Running), setup)
        .add_systems(
            Update,
            (patch_lights, player_move, camera_follow_player).run_if(in_state(GameState::Running)),
        )
        .insert_resource(AmbientLight {
            brightness: 30.0,
            ..default()
        })
        .run();
}

#[derive(Component)]
struct Player;

fn setup(mut commands: Commands, image_assets: Res<ImageAssets>) {
    // cube
    commands
        .spawn((
            Player,
            Transform::from_xyz(0.0, 0.5, 0.0),
            Collider::cuboid(0.5, 0.5, 0.5),
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
        ))
        .with_child((
            SceneRoot(image_assets.chicken.clone()),
            Transform::from_scale(Vec3::splat(0.1)),
        ));

    commands.spawn((Car, Transform::from_translation(Vec3::new(-3.0, 0.0, 0.0))));

    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 12.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        // Projection::Orthographic(OrthographicProjection {
        //     scale: 0.01,
        //     ..OrthographicProjection::default_3d()
        // }),
    ));
}

fn player_move(mut player: Query<&mut Transform, With<Player>>, keys: Res<ButtonInput<KeyCode>>) {
    let mut player = player.single_mut();
    let old_translation = player.translation;
    for key in keys.get_pressed() {
        match key {
            KeyCode::ArrowRight => player.translation.x += 0.02,
            KeyCode::ArrowLeft => player.translation.x -= 0.02,
            KeyCode::ArrowUp => player.translation.z -= 0.02,
            KeyCode::ArrowDown => player.translation.z += 0.02,
            _ => {}
        }
    }

    let dtrans = old_translation - player.translation;
    let mut player_copy = player.clone();
    if dtrans.length() > 0.001 {
        player_copy.look_to(dtrans, Vec3::Y);
    }

    player.rotation = player.rotation.lerp(player_copy.rotation, 0.15);
}

fn camera_follow_player(
    mut transforms: Query<&mut Transform>,
    player: Query<Entity, With<Player>>,
    camera: Query<Entity, With<Camera3d>>,
) {
    let player = player.single();
    let camera = camera.single();
    let Ok(player_transform) = transforms.get(player).cloned() else {
        return;
    };
    let Ok(mut camera_transform) = transforms.get_mut(camera) else {
        return;
    };

    let target =
        Transform::from_translation(player_transform.translation + Vec3::new(0.0, 12.5, 5.0))
            .looking_at(player_transform.translation, Vec3::Y);

    camera_transform.translation = camera_transform.translation.lerp(target.translation, 0.05);
    camera_transform.rotation = camera_transform.rotation.lerp(target.rotation, 0.05);
}

fn patch_lights(mut q: Query<&mut Transform, With<SpotLight>>) {
    for mut t in q.iter_mut() {
        t.translation.x += 0.00000001;
    }
}
