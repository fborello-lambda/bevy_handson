use bevy::prelude::*;

mod components;
mod fish;
mod algae;
mod resources;
use resources::*;
use algae::*;
use fish::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, camera_setup)
        .add_plugins(FishPlugin)
        .add_plugins(AlgaePlugin)
        .add_plugins(TimerPlugin)
        .run();
}

fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
