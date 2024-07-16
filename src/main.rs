use rand::Rng;
use bevy::{math::bounding::Aabb2d, prelude::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, fish_movement)
        .run();
}

#[derive(Component)]
struct Fish{
    velocity: Vec2,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((SpriteBundle {
        texture: asset_server.load("fish.png"),
        transform: Transform::from_scale(Vec3 { x: 5.0, y: 5.0, z: 5.0 }),
        ..default()
    }, Fish{
        velocity: Vec2::ZERO,
    }));
    commands.spawn((SpriteBundle {
        texture: asset_server.load("fish.png"),
        transform: Transform::from_scale(Vec3 { x: 5.0, y: 5.0, z: 5.0 }),
        ..default()
    }, Fish{
        velocity: Vec2::ZERO,
    }));
    commands.spawn((SpriteBundle {
        texture: asset_server.load("fish.png"),
        transform: Transform::from_scale(Vec3 { x: 5.0, y: 5.0, z: 5.0 }),
        ..default()
    }, Fish{
        velocity: Vec2::ZERO,
    }));
}

fn fish_movement(time: Res<Time>, windows: Query<&Window>,mut fish_query: Query<(&mut Fish, &mut Transform)>){
    let window = windows.single();
    let window_size = window.size();
    let collision_area = Aabb2d::new(Vec2::ZERO, (window_size - 5.0) / 2.);


    for (mut fish, mut transform) in &mut fish_query{
        transform.translation.x += fish.velocity.x * time.delta_seconds();
        transform.translation.y += fish.velocity.y * time.delta_seconds();

        transform.translation.x = transform.translation.x.clamp(collision_area.min.x, collision_area.max.x);
        transform.translation.y = transform.translation.y.clamp(collision_area.min.y, collision_area.max.y);

        let v = fish.velocity.length();
        let acc = 0.01 * v * v * time.delta_seconds();
        fish.velocity = fish.velocity.normalize_or_zero() * (v - acc);

        if fish.velocity.length() < 20.{
            let mut rng = rand::thread_rng();
            fish.velocity = Vec2 {
                x: rng.gen_range(-1.0..=1.0),
                y: rng.gen_range(-1.0..=1.0),
            };
            fish.velocity *= rng.gen_range(100.0..=400.0);
        }
    }
}


