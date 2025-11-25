use crate::ball::Ball;
use avian2d::prelude::*;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(bounce_objects)
        .add_systems(FixedPostUpdate, kinematic_wall_stop_system);
}

#[derive(Component, Default)]
#[require(CollisionEventsEnabled)]
pub struct BounceCollider(pub f32);

fn bounce_objects(
    event: On<CollisionStart>,
    collisions: Collisions,
    bounce_query: Query<&BounceCollider>,
    mut velocity_query: Query<&mut LinearVelocity, With<Ball>>,
) {
    let bounce_entity = event.collider1;
    let target_entity = event.collider2;

    let Ok(bounce) = bounce_query.get(bounce_entity) else { return };
    let Ok(mut linear_velocity) = velocity_query.get_mut(target_entity) else { return };
    let Some(contact_pair) = collisions.get(bounce_entity, target_entity) else { return };
    let Some(manifold) = contact_pair.manifolds.first() else { return };

    let normal = manifold.normal;
    let dot_product = linear_velocity.0.dot(normal);

    if dot_product < 0.0 {
        // Only on approach (safety for ongoing contacts)
        linear_velocity.0 = linear_velocity.0.reflect(normal) * bounce.0;
    }
}

use avian2d::prelude::*;
use bevy::prelude::*;

/// A component that forces a Kinematic body to stop when it detects a hit
/// via its ShapeCaster.
///
/// Requires: RigidBody::Kinematic, LinearVelocity, and a ShapeCaster.
#[derive(Component, Default)]
#[require(RigidBody)]
pub struct KinematicWallStop {
    /// The small buffer distance to keep from the wall to prevent
    /// numerical floating point issues (stuck inside wall).
    /// Default: 0.01
    pub skin_width: f32,
}

impl KinematicWallStop {
    pub fn new(skin_width: f32) -> Self {
        Self { skin_width }
    }
}

/// The system that handles the collision logic
pub fn kinematic_wall_stop_system(
    time: Res<Time>,
    mut query: Query<(
        &mut Transform,
        &mut LinearVelocity,
        &mut ShapeCaster,
        &ShapeHits,
        &KinematicWallStop,
    )>,
) {
    let delta_seconds = time.delta_secs();

    for (mut transform, mut velocity, mut caster, hits, settings) in &mut query {
        // 1. If we aren't moving, we don't need to cast rays
        if velocity.length_squared() < 0.001 {
            continue;
        }

        // 2. Update the ShapeCaster to look where we are going
        // We cast slightly further than we plan to move to ensure we catch the wall
        let movement_direction = velocity.normalize_or_zero();
        let planned_speed = velocity.length();
        let planned_distance = planned_speed * delta_seconds;

        caster.direction = Dir2::new(movement_direction).unwrap_or(Dir2::X);
        // We cast strictly the distance we plan to travel + a tiny check margin
        caster.max_distance = planned_distance + settings.skin_width;

        // 3. Check for hits
        // We iter hits to find the closest valid obstruction
        for hit in hits.iter() {
            // "time_of_impact" in Avian ShapeCasting is essentially "distance"
            // if the direction is normalized.

            // If the wall is closer than where we plan to be at the end of the frame:
            if hit.distance < planned_distance {
                // A. Prevent Overlap / Snap to surface
                // Move exactly to the impact point minus the skin width
                let safe_distance = (hit.distance - settings.skin_width).max(0.0);

                // Manually apply the safe translation immediately
                transform.translation += (movement_direction * safe_distance).extend(0.0);

                // B. Stop the Velocity
                // For a platformer, you might project velocity (slide).
                // For Pong, we just want to stop dead.
                *velocity = LinearVelocity(Vec2::ZERO);

                // Since we stopped, we break the loop (don't process other hits)
                break;
            }
        }
    }
}
