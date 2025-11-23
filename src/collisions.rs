use avian2d::prelude::*;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(bounce_objects);
}

#[derive(Component, Default)]
#[require(CollisionEventsEnabled)]
pub struct BounceCollider(pub f32);

fn bounce_objects(
    event: On<CollisionStart>,
    collisions: Collisions,
    bounce_query: Query<&BounceCollider>,
    mut velocity_query: Query<&mut LinearVelocity>,
) {
    let collider1 = event.collider1;
    let collider2 = event.collider2;

    // Symmetric: Check which is bounce, which has velocity to modify
    let (bounce_entity, target_entity) = if bounce_query.contains(collider1) {
        (collider1, collider2)
    } else if bounce_query.contains(collider2) {
        (collider2, collider1)
    } else {
        return;
    };

    let Ok(bounce) = bounce_query.get(bounce_entity) else {
        return;
    };

    let Ok(mut linear_velocity) = velocity_query.get_mut(target_entity) else {
        return;
    };

    // Get the specific contact pair for this event
    let Some(contact_pair) = collisions.get(collider1, collider2) else {
        return;
    };

    let Some(manifold) = contact_pair.manifolds.first() else {
        return;
    };

    let normal = manifold.normal;
    let dot_product = linear_velocity.0.dot(normal);
    if dot_product > 0.0 {
        return;
    }

    // Only on approach (safety for ongoing contacts)
    println!("Bounce! Velocity: {:?}", linear_velocity.0);
    linear_velocity.0 = linear_velocity.0.reflect(normal) * bounce.0;
}
