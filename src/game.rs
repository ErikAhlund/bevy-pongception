use bevy::prelude::*;

// Declare all modules
mod ball;
mod collisions;
mod components;
mod paddle;
mod score;
mod setup;

pub fn plugin(app: &mut App) {
    app.add_plugins(DefaultPlugins)
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .add_plugins(ball::plugin)
        .add_plugins(paddle::plugin)
        .add_plugins(score::plugin)
        .add_plugins(collisions::plugin)
        .add_plugins(setup::plugin);
}
