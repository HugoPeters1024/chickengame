use std::f32::consts::PI;

use assets::ImageAssets;
use bevy::{math::Affine2, prelude::*};
use bevy_asset_loader::prelude::*;
use lantern::{Lantern, LanternPlugin};
use street::StreetPlugin;

mod assets;
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
        .add_plugins((LanternPlugin, StreetPlugin))
        .init_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::Running)
                .load_collection::<ImageAssets>(),
        )
        .add_systems(OnEnter(GameState::Running), setup)
        .add_systems(
            Update,
            (patch_lights, player_move, camera_follow_player, cars_drive)
                .run_if(in_state(GameState::Running)),
        )
        .run();
}

fn hex_color(color: &str) -> Color {
    Color::Srgba(Srgba::hex(color).unwrap())
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Car;

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    image_assets: Res<ImageAssets>,
) {
    // floor
    //commands.spawn((
    //    Mesh3d(meshes.add(Cuboid::new(100.0, 0.1, 10.0))),
    //    MeshMaterial3d(materials.add(StandardMaterial {
    //        base_color_texture: Some(image_assets.road.clone()),
    //        uv_transform: Affine2::from_scale_angle_translation(
    //            Vec2::new(20.0, 2.0),
    //            PI / 2.0,
    //            Vec2::ZERO,
    //        ),
    //        ..default()
    //    })),
    //    Transform::from_xyz(0.0, -0.1, 0.0),
    //));

    // cube
    commands.spawn((
        Player,
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));

    commands.spawn((
        SceneRoot(image_assets.car.clone()),
        Car,
        Transform::from_translation(Vec3::new(0.0, 0.0, 2.0)),
    ));

    for i in 0..10 {
        commands.spawn((
            Lantern,
            Transform::from_rotation(Quat::from_rotation_y(-PI / 2.0)).with_translation(Vec3::new(
                3.0 * i as f32,
                0.0,
                0.0,
            )),
        ));
    }

    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 12.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        //Projection::Orthographic(OrthographicProjection {
        //    scale: 0.01,
        //    ..OrthographicProjection::default_3d()
        //}),
    ));
}

fn player_move(mut player: Query<&mut Transform, With<Player>>, keys: Res<ButtonInput<KeyCode>>) {
    let mut player = player.single_mut();
    for key in keys.get_pressed() {
        match key {
            KeyCode::ArrowRight => player.translation.x += 0.02,
            KeyCode::ArrowLeft => player.translation.x -= 0.02,
            KeyCode::ArrowUp => player.translation.z -= 0.02,
            KeyCode::ArrowDown => player.translation.z += 0.02,
            _ => {}
        }
    }
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

    *camera_transform =
        Transform::from_translation(player_transform.translation + Vec3::new(0.0, 12.5, 5.0))
            .looking_at(player_transform.translation, Vec3::Y);
}

fn cars_drive(mut q: Query<&mut Transform, With<Car>>) {
    for mut t in q.iter_mut() {
        t.translation.x += 0.02;
    }
}

fn patch_lights(mut q: Query<&mut Transform, With<PointLight>>) {
    for mut t in q.iter_mut() {
        t.translation.x += 0.00000001;
    }
}
