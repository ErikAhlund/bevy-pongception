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
