use crate::components::{PongCollider, PongCollision, PongPosition};
use crate::paddle::Paddle;
use bevy::math::bounding::{Aabb2d, BoundingVolume, IntersectsVolume};
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(FixedUpdate, (project_positions, constrain_paddle_position));
}

pub fn project_positions(positions: Query<(&mut Transform, &PongPosition)>) {
    for (mut transform, position) in positions {
        transform.translation = position.0.extend(0.);
    }
}

pub fn collide_with_side(ball: Aabb2d, wall: Aabb2d) -> Option<PongCollision> {
    if !ball.intersects(&wall) {
        return None;
    }

    let closest_point = wall.closest_point(ball.center());
    let offset = ball.center() - closest_point;

    let side = if offset.x.abs() > offset.y.abs() {
        if offset.x < 0. {
            PongCollision::Left
        } else {
            PongCollision::Right
        }
    } else if offset.y > 0. {
        PongCollision::Top
    } else {
        PongCollision::Bottom
    };

    Some(side)
}

pub fn constrain_paddle_position(
    mut paddles: Query<
        (&mut PongPosition, &PongCollider),
        (With<Paddle>, Without<crate::paddle::Gutter>),
    >,
    gutters: Query<(&PongPosition, &PongCollider), (With<crate::paddle::Gutter>, Without<Paddle>)>,
) {
    for (mut paddle_position, paddle_collider) in &mut paddles {
        for (gutter_position, gutter_collider) in &gutters {
            let paddle_aabb = Aabb2d::new(paddle_position.0, paddle_collider.half_size());
            let gutter_aabb = Aabb2d::new(gutter_position.0, gutter_collider.half_size());

            if let Some(collision) = collide_with_side(paddle_aabb, gutter_aabb) {
                match collision {
                    PongCollision::Top => {
                        paddle_position.0.y = gutter_position.0.y
                            + gutter_collider.half_size().y
                            + paddle_collider.half_size().y;
                    }
                    PongCollision::Bottom => {
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
