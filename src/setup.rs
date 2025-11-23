use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.insert_resource(Time::<Fixed>::from_hz(60.0))
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
