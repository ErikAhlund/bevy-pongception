use crate::components::{Collider, Collision, Position};
use crate::paddle::Paddle;
use bevy::math::bounding::{Aabb2d, BoundingVolume, IntersectsVolume};
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(FixedUpdate, (project_positions, constrain_paddle_position));
}

pub fn project_positions(positions: Query<(&mut Transform, &Position)>) {
    for (mut transform, position) in positions {
        transform.translation = position.0.extend(0.);
    }
}

pub fn collide_with_side(ball: Aabb2d, wall: Aabb2d) -> Option<Collision> {
    if !ball.intersects(&wall) {
        return None;
    }

    let closest_point = wall.closest_point(ball.center());
    let offset = ball.center() - closest_point;

    let side = if offset.x.abs() > offset.y.abs() {
        if offset.x < 0. {
            Collision::Left
        } else {
            Collision::Right
        }
    } else if offset.y > 0. {
        Collision::Top
    } else {
        Collision::Bottom
    };

    Some(side)
}

pub fn constrain_paddle_position(
    mut paddles: Query<(&mut Position, &Collider), (With<Paddle>, Without<crate::paddle::Gutter>)>,
    gutters: Query<(&Position, &Collider), (With<crate::paddle::Gutter>, Without<Paddle>)>,
) {
    for (mut paddle_position, paddle_collider) in &mut paddles {
        for (gutter_position, gutter_collider) in &gutters {
            let paddle_aabb = Aabb2d::new(paddle_position.0, paddle_collider.half_size());
            let gutter_aabb = Aabb2d::new(gutter_position.0, gutter_collider.half_size());

            if let Some(collision) = collide_with_side(paddle_aabb, gutter_aabb) {
                match collision {
                    Collision::Top => {
                        paddle_position.0.y = gutter_position.0.y
                            + gutter_collider.half_size().y
                            + paddle_collider.half_size().y;
                    }
                    Collision::Bottom => {
                        paddle_position.0.y = gutter_position.0.y
                            - gutter_collider.half_size().y
                            - paddle_collider.half_size().y;
                    }
                    _ => {}
                }
            }
        }
    }
}
