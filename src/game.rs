use bevy::prelude::*;

// Declare all modules
mod ball;
mod collisions;
mod debug;
mod paddle;
mod score;
mod setup;

mod components;

pub fn plugin(app: &mut App) {
    app.add_plugins(setup::plugin)
        .add_plugins(ball::plugin)
        .add_plugins(paddle::plugin)
        .add_plugins(score::plugin)
        .add_plugins(debug::plugin)
        .add_plugins(collisions::plugin);
}
