use bevy::{math::bounding::Aabb2d, prelude::*};
use rand::*;

use crate::components::*;

#[derive(Bundle)]
pub struct Fish {
    pub(crate) velocity: Velocity,
    pub(crate) position: Position,
    pub(crate) objective: Objective,
    pub(crate) sprite: SpriteBundle,
}

pub struct FishPlugin;
impl Plugin for FishPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, fish_setup)
            .add_systems(Update, fish_update);
    }
}

fn fish_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    for _ in 0..1 {
        let start_pos = Vec2::new(
            rand::thread_rng().gen_range(-300.0..300.0),
            rand::thread_rng().gen_range(-300.0..300.0),
        );
        commands.spawn(Fish {
            position: Position { v2: start_pos },
            velocity: Velocity { v2: Vec2::ZERO },
            objective: Objective { v2: start_pos },
            sprite: SpriteBundle {
                texture: asset_server.load("fish-forage/fish-sardine.png"),
                transform: Transform::from_scale(Vec3::splat(2.)),
                ..default()
            },
        });
    }
}

fn fish_update(
    time: Res<Time>,
    windows: Query<&Window>,
    mut fish_query: Query<(&mut Velocity, &mut Position, &mut Objective, &mut Transform)>,
) {
    let window = windows.single();
    let window_size = window.size();
    let collision_area = Aabb2d::new(Vec2::ZERO, (window_size - 5.0) / 2.);

    for (velocity, position, objective, mut transform) in &mut fish_query {
        let mut v = velocity.v2;
        let mut p = position.v2;
        let o = objective.v2;

        p += v * time.delta_seconds();
        p.x = p.x.clamp(collision_area.min.x, collision_area.max.x);
        p.y = p.y.clamp(collision_area.min.y, collision_area.max.y);

        let acc = 1. * (o - p).normalize_or_zero();
        v += acc * time.delta_seconds();

        v = v.clamp_length_max(80.);

        transform.translation += Vec3::new(p.x, p.y, 0.);

        println!(
            "position: {:?}, velocity: {:?}, objective {:?}, timer: {:?}",
            p,
            v,
            o,
            time.delta_seconds()
        );
    }
}
