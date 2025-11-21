mod ball;
mod collisions;
mod components;
mod paddle;
mod score;

use crate::ball::*;
use crate::collisions::*;
use crate::components::*;
use crate::paddle::*;
use crate::score::*;
use bevy::camera::*;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Score { player: 0, ai: 0 })
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .add_systems(
            Startup,
            (
                spawn_ball,
                spawn_paddles,
                spawn_camera,
                spawn_gutters,
                spawn_scoreboard,
            ),
        )
        .add_systems(
            FixedUpdate,
            (
                project_positions,
                move_ball,
                handle_collisions,
                move_paddles,
                handle_player_input,
                constrain_paddle_position,
                detect_goal,
                update_scoreboard,
                move_ai,
            ),
        )
        .add_observer(reset_ball)
        .add_observer(update_score)
        .run();
}

fn spawn_camera(mut commands: Commands, window: Single<&Window>) {
    commands.spawn((
        Camera2d,
        Projection::from(OrthographicProjection {
            scaling_mode: bevy::camera::ScalingMode::AutoMin {
                min_width: window.resolution.width(),
                min_height: window.resolution.height(),
            },
            ..OrthographicProjection::default_2d()
        }),
    ));
}

pub fn project_positions(positions: Query<(&mut Transform, &Position)>) {
    for (mut transform, position) in positions {
        transform.translation = position.0.extend(0.);
    }
}
