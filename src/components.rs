use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct Velocity {
    pub v2: Vec2,
}

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub v2: Vec2,
}

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct Objective {
    pub v2: Vec2,
}
