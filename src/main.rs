use std::time::Duration;

use bevy::{math::bounding::Aabb2d, prelude::*};
use rand::Rng;

pub struct FishPlugin;
impl Plugin for FishPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, fish_setup)
            .add_systems(Update, fish_update);
    }
}

pub struct AlgaePlugin;
impl Plugin for AlgaePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, algae_setup)
            .add_systems(Update, algae_update);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, camera_setup)
        .add_plugins(FishPlugin)
        .add_plugins(AlgaePlugin)
        .add_plugins(TimerPlugin)
        .run();
}

#[derive(Component, Debug, Clone, Copy, PartialEq)]
struct Velocity {
    v2: Vec2,
}

#[derive(Component, Debug, Clone, Copy, PartialEq)]
struct Position {
    v2: Vec2,
}

#[derive(Component, Debug, Clone, Copy, PartialEq)]
struct Objective {
    v2: Vec2,
}

#[derive(Bundle)]
struct Fish {
    velocity: Velocity,
    position: Position,
    objective: Objective,
    sprite: SpriteBundle,
}

#[derive(Bundle)]
struct Algae {
    position: Position,
    sprite: SpriteBundle,
}

#[derive(Resource)]
struct ChangeObjectiveTimer(Timer);

fn update_timer(mut timer: ResMut<ChangeObjectiveTimer>) {
    timer.0.tick(Duration::from_secs_f32(5.0));
}

struct TimerPlugin;

impl Plugin for TimerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ChangeObjectiveTimer(Timer::from_seconds(
            5.0,
            TimerMode::Repeating,
        )))
        .add_systems(Update, update_timer);
    }
}

fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
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

fn algae_update(
    time: Res<Time>,
    mut timer: ResMut<ChangeObjectiveTimer>,
    mut objective_query: Query<&mut Objective>,
    //mut algae_query: Query<&mut Transform, With<Position>>,
) {
    let new_pos = Vec2::new(
        rand::thread_rng().gen_range(-300.0..300.0),
        rand::thread_rng().gen_range(-300.0..300.0),
    );
    if timer.0.tick(time.delta()).finished() {
        //for mut algae in &mut algae_query {
        //    algae.translation = Vec3::new(new_pos.x, new_pos.y, 0.);
        //}
        for mut o in &mut objective_query {
            o.v2 = new_pos;
        }
    }
}
