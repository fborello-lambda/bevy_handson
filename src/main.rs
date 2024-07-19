use bevy::prelude::*;
use bevy_embedded_assets::*;

mod algae;
mod components;
mod fish;
mod resources;
use algae::*;
use fish::*;
use resources::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EmbeddedAssetPlugin::default())
        .add_systems(Startup, camera_setup)
        .add_plugins(FishPlugin)
        .add_plugins(AlgaePlugin)
        .add_plugins(TimerPlugin)
        .run();
}

fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
