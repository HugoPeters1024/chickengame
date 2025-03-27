use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct ImageAssets {
    #[asset(path = "sprites/road.jpg")]
    #[asset(image(sampler(filter = linear, wrap=repeat)))]
    pub road: Handle<Image>,

    #[asset(path = "models/toycar.glb#Scene0")]
    pub car: Handle<Scene>,

    #[asset(path = "models/lantern.glb#Scene0")]
    pub lantern: Handle<Scene>,
}
