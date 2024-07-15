use std::time::Duration;

use bevy::{math::bounding::Aabb2d, prelude::*};
use rand::Rng;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (swim, move_algae))
        .insert_resource(ChangeObjectiveTimer(Timer::from_seconds(
            5.0,
            TimerMode::Repeating,
        )))
        .run();
}

#[derive(Component)]
struct Swimmer {
    position: Vec2,
    velocity: Vec2,
    objective: Vec2,
}

#[derive(Resource)]
struct ChangeObjectiveTimer(Timer);

#[derive(Component)]
struct Algae;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut timer: ResMut<ChangeObjectiveTimer>,
) {
    timer.0.tick(Duration::from_secs_f32(4.9));

    commands.spawn(Camera2dBundle::default());
    for _ in 0..10 {
        let start_pos = Vec2::new(
            rand::thread_rng().gen_range(-300.0..300.0),
            rand::thread_rng().gen_range(-300.0..300.0),
        );
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("fish-forage/fish-sardine.png"),
                transform: Transform::from_scale(Vec3::splat(2.)),
                ..default()
            },
            Swimmer {
                position: start_pos,
                velocity: Vec2::ZERO,
                objective: start_pos,
            },
        ));
    }

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("fish-forage/algae.png"),
            transform: Transform::from_scale(Vec3::splat(2.)),
            ..default()
        },
        Algae,
    ));
}

fn swim(
    time: Res<Time>,
    windows: Query<&Window>,
    mut fish_query: Query<(&mut Swimmer, &mut Transform)>,
) {
    let window = windows.single();
    let window_size = window.size();
    let collision_area = Aabb2d::new(Vec2::ZERO, (window_size - 5.0) / 2.);

    for (mut swimmer, mut transform) in &mut fish_query {
        let v = swimmer.velocity;
        swimmer.position += v * time.delta_seconds();

        swimmer.position.x = swimmer
            .position
            .x
            .clamp(collision_area.min.x, collision_area.max.x);
        swimmer.position.y = swimmer
            .position
            .y
            .clamp(collision_area.min.y, collision_area.max.y);

        let acc = 100. * (swimmer.objective - swimmer.position).normalize_or_zero();
        swimmer.velocity += acc * time.delta_seconds();

        swimmer.velocity = swimmer.velocity.clamp_length_max(80.);

        transform.translation = Vec3::new(swimmer.position.x, swimmer.position.y, 0.);

        println!(
            "position: {:?}, velocity: {:?}",
            swimmer.position, swimmer.velocity
        );
    }
}

fn move_algae(
    time: Res<Time>,
    mut timer: ResMut<ChangeObjectiveTimer>,
    mut swimmer_query: Query<&mut Swimmer>,
    mut algae_query: Query<&mut Transform, With<Algae>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for mut algae in &mut algae_query {
            let new_pos = Vec2::new(
                rand::thread_rng().gen_range(-300.0..300.0),
                rand::thread_rng().gen_range(-300.0..300.0),
            );
            algae.translation = Vec3::new(new_pos.x, new_pos.y, 0.);
            for mut swimmer in &mut swimmer_query {
                swimmer.objective = new_pos;
            }
        }
    }
}
