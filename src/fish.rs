use bevy::{math::bounding::Aabb2d, prelude::*};
use rand::*;

use crate::components::*;

#[derive(Component)]
pub struct Fish;
#[derive(Bundle)]
pub struct FishBundle {
    pub(crate) velocity: Velocity,
    pub(crate) position: Position,
    pub(crate) objective: Objective,
    pub(crate) sprite: SpriteBundle,
    pub(crate) fish_component: Fish,
}

pub struct FishPlugin;
impl Plugin for FishPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, fish_setup)
            .add_systems(Update, fish_update);
    }
}

fn fish_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    for _ in 0..10 {
        let start_pos = Vec2::new(
            rand::thread_rng().gen_range(-300.0..300.0),
            rand::thread_rng().gen_range(-300.0..300.0),
        );
        commands.spawn(FishBundle {
            position: Position { v2: start_pos },
            velocity: Velocity { v2: Vec2::ZERO },
            objective: Objective { v2: start_pos },
            sprite: SpriteBundle {
                texture: asset_server.load("embedded://fish-forage/fish-sardine.png"),
                transform: Transform::from_scale(Vec3::splat(2.)),
                ..default()
            },
            fish_component: Fish,
        });
    }
}

fn fish_update(
    time: Res<Time>,
    windows: Query<&Window>,
    algae_query: Query<&Position, Without<Fish>>,
    mut fish_query: Query<
        (&mut Velocity, &mut Position, &mut Objective, &mut Transform),
        With<Fish>,
    >,
) {
    let window = windows.single();
    let window_size = window.size();
    let collision_area = Aabb2d::new(Vec2::ZERO, (window_size - 5.0) / 2.);

    let mut algae_iter = algae_query.iter();

    for (mut v, mut p, mut o, mut transform) in &mut fish_query {
        if let Some(algae_position) = algae_iter.next() {
            o.v2 = algae_position.v2;
        } else {
            // If there are more fish than algae, reset the algae iterator
            algae_iter = algae_query.iter();
            if let Some(algae_position) = algae_iter.next() {
                o.v2 = algae_position.v2;
            }
        }

        p.v2 += v.v2 * time.delta_seconds();
        p.v2.x = p.v2.x.clamp(collision_area.min.x, collision_area.max.x);
        p.v2.y = p.v2.y.clamp(collision_area.min.y, collision_area.max.y);

        let acc = 100. * (o.v2 - p.v2).normalize_or_zero();
        v.v2 += acc * time.delta_seconds();

        v.v2 = v.v2.clamp_length_max(80.);

        transform.translation = Vec3::new(p.v2.x, p.v2.y, 0.);
    }
}
