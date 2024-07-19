use bevy::prelude::*;
use std::time::Duration;

#[derive(Resource)]
pub struct ChangeObjectiveTimer(pub Timer);

pub struct TimerPlugin;

impl Plugin for TimerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ChangeObjectiveTimer(Timer::from_seconds(
            5.0,
            TimerMode::Repeating,
        )))
        .add_systems(Update, update_timer);
    }
}

fn update_timer(mut timer: ResMut<ChangeObjectiveTimer>) {
    timer.0.tick(Duration::from_secs_f32(5.0));
}
