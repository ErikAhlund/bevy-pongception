use bevy::prelude::*;

#[derive(Component, Default)]
#[require(Transform)]
pub struct Position(pub Vec2);

#[derive(Component, Default)]
pub struct Velocity(pub Vec2);

#[derive(Component, Default)]
pub struct Collider(pub Rectangle);

impl Collider {
    pub fn half_size(&self) -> Vec2 {
        self.0.half_size
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Collision {
    Left,
    Right,
    Top,
    Bottom,
}
