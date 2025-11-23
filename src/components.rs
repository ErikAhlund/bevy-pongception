use bevy::prelude::*;

#[derive(Component, Default)]
#[require(Transform)]
pub struct PongPosition(pub Vec2);

#[derive(Component, Default)]
pub struct PongVelocity(pub Vec2);

#[derive(Component, Default)]
pub struct PongCollider(pub Rectangle);

impl PongCollider {
    pub fn half_size(&self) -> Vec2 {
        self.0.half_size
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum PongCollision {
    Left,
    Right,
    Top,
    Bottom,
}
