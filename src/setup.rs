use avian2d::prelude::*;
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_plugins((DefaultPlugins, PhysicsPlugins::default()))
        .insert_resource(Gravity(Vec2::NEG_Y * 1000.0))
        .insert_resource(Time::<Fixed>::from_hz(64.0))
        .add_systems(Startup, spawn_camera);
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
