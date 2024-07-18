use crate::components::*;
use crate::resources::*;
use bevy::prelude::*;
use rand::*;

#[derive(Component)]
pub struct Algae;
#[derive(Bundle)]
pub struct AlgaeBundle {
    pub(crate) position: Position,
    pub(crate) sprite: SpriteBundle,
    pub(crate) algae_component: Algae,
}

pub struct AlgaePlugin;
impl Plugin for AlgaePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, algae_setup)
            .add_systems(Update, algae_update);
    }
}

fn algae_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    for _ in 0..10 {
        let start_pos = Vec2::new(
            rand::thread_rng().gen_range(-300.0..300.0),
            rand::thread_rng().gen_range(-300.0..300.0),
        );
        commands.spawn(AlgaeBundle {
            algae_component: Algae,
            position: Position { v2: start_pos },
            sprite: SpriteBundle {
                texture: asset_server.load("embedded://fish-forage/algae.png"),
                transform: Transform::from_scale(Vec3::splat(1.)),
                ..default()
            },
        });
    }
}

fn algae_update(
    time: Res<Time>,
    mut timer: ResMut<ChangeObjectiveTimer>,
    mut algae_query: Query<(&mut Transform, &mut Position), With<Algae>>,
) {
    if timer.0.tick(time.delta()).finished() {
        for (mut transform, mut position) in &mut algae_query {
            let new_pos = Vec2::new(
                rand::thread_rng().gen_range(-300.0..300.0),
                rand::thread_rng().gen_range(-300.0..300.0),
            );
            transform.translation = Vec3::new(new_pos.x, new_pos.y, 0.);
            position.v2 = new_pos;
        }
    }
}
