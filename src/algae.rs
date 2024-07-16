use bevy::prelude::*;
use rand::*;
use crate::components::*;
use crate::resources::*;

#[derive(Bundle)]
pub struct Algae {
    pub(crate) position: Position,
    pub(crate) sprite: SpriteBundle,
}

pub struct AlgaePlugin;
impl Plugin for AlgaePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, algae_setup)
            .add_systems(Update, algae_update);
    }
}

fn algae_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let start_pos = Vec2::new(
        rand::thread_rng().gen_range(-300.0..300.0),
        rand::thread_rng().gen_range(-300.0..300.0),
    );
    commands.spawn(Algae {
        position: Position { v2: start_pos },
        sprite: SpriteBundle {
            texture: asset_server.load("fish-forage/algae.png"),
            transform: Transform::from_scale(Vec3::splat(2.)),
            ..default()
        },
    });
}

fn algae_update(
    time: Res<Time>,
    mut timer: ResMut<ChangeObjectiveTimer>,
    mut objective_query: Query<&mut Objective>,
    mut algae_query: Query<&mut Transform, With<Position>>,
) {
    let new_pos = Vec2::new(
        rand::thread_rng().gen_range(-300.0..300.0),
        rand::thread_rng().gen_range(-300.0..300.0),
    );
    if timer.0.tick(time.delta()).finished() {
        for mut algae in &mut algae_query {
            algae.translation = Vec3::new(new_pos.x, new_pos.y, 0.);
        }
        for mut o in &mut objective_query {
            o.v2 = new_pos;
        }
    }
}
