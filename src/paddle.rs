use crate::ball::Ball;
use avian2d::math::{AdjustPrecision, AsF32};
use avian2d::prelude::*;
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, (spawn_paddles, spawn_gutters))
        .add_systems(FixedUpdate, (move_ai, move_paddles));
}

const PADDLE_WIDTH: f32 = 50.0;
const PADDLE_HEIGHT: f32 = 200.0;

#[derive(Component, Default)]
#[require(
    RigidBody::Kinematic,
    Collider::rectangle(PADDLE_WIDTH, PADDLE_HEIGHT),
    TransformInterpolation
)]
pub struct Paddle {
    pub velocity: Vec2,
}

#[derive(Component)]
#[require(RigidBody::Static)]
pub struct Gutter;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Ai;

const PADDLE_SHAPE: Rectangle = Rectangle::new(PADDLE_WIDTH, PADDLE_HEIGHT);
const PADDLE_COLOR: Color = Color::srgb(0., 1., 0.);

fn spawn_paddles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Single<&Window>,
) {
    let mesh = meshes.add(PADDLE_SHAPE);
    let material = materials.add(PADDLE_COLOR);
    let half_window_size = window.resolution.size() / 2.;
    let padding = 20.;
    let player_position = Vec2::new(-half_window_size.x + padding, 0.);

    commands.spawn((
        Player,
        Paddle::default(),
        Mesh2d(mesh.clone()),
        MeshMaterial2d(material.clone()),
        Position(player_position),
    ));

    let ai_position = Vec2::new(half_window_size.x - padding, 0.);
    commands.spawn((
        Ai,
        Paddle::default(),
        Mesh2d(mesh.clone()),
        MeshMaterial2d(material.clone()),
        Position(ai_position),
    ));
}

const GUTTER_COLOR: Color = Color::srgb(0., 0., 1.);
const GUTTER_HEIGHT: f32 = 20.;

fn spawn_gutters(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Single<&Window>,
) {
    let material = materials.add(GUTTER_COLOR);

    let gutter_shape = Rectangle::new(window.resolution.width(), GUTTER_HEIGHT);
    let mesh = meshes.add(gutter_shape);

    let half_window = window.resolution.height() / 2.;

    commands.spawn((
        Gutter,
        Position::from_xy(0.0, half_window),
        Collider::half_space(Vec2::NEG_Y),
        Mesh2d(mesh.clone()),
        MeshMaterial2d(material.clone()),
    ));

    commands.spawn((
        Gutter,
        Position::from_xy(0.0, -half_window),
        Collider::half_space(Vec2::Y),
        Mesh2d(mesh.clone()),
        MeshMaterial2d(material.clone()),
    ));
}

const PADDLE_MAX_SPEED: f32 = 1000.0; // Maximum speed when far away
const PADDLE_MIN_SPEED: f32 = 10.0; // Minimum speed when getting close
const PADDLE_DEADZONE: f32 = 5.0; // Stop completely within this range

fn move_paddles(
    paddles: Query<(Entity, &mut Transform, &mut Paddle, &Collider), With<Paddle>>,
    move_and_slide: MoveAndSlide,
    time: Res<Time>,
) {
    for (entity, mut transform, mut player, collider) in paddles {
        // Perform move and slide
        let MoveAndSlideOutput {
            position,
            projected_velocity: velocity,
        } = move_and_slide.move_and_slide(
            collider,
            transform.translation.xy().adjust_precision(),
            transform.rotation.to_euler(EulerRot::XYZ).2,
            player.velocity,
            time.delta(),
            &MoveAndSlideConfig::default(),
            &SpatialQueryFilter::from_excluded_entities([entity]),
            |_hit| true,
        );

        // Update transform and stored velocity
        transform.translation = position.extend(0.0).f32();
        player.velocity = velocity.f32();
    }
}

fn move_ai(mut paddles: Query<(&mut Paddle, &Position)>, ball: Single<&Position, With<Ball>>) {
    let ball_y = ball.y;

    for (mut paddle, position) in &mut paddles {
        let difference = ball_y - position.y;
        let distance = difference.abs();
        if distance > PADDLE_DEADZONE {
            let direction = difference.signum();

            let speed =
                PADDLE_MIN_SPEED + (PADDLE_MAX_SPEED - PADDLE_MIN_SPEED) * (distance / 100.0);

            paddle.velocity.y = direction * speed;
        } else {
            paddle.velocity.y = 0.0;
        }
    }
}
