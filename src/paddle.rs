use super::collisions::BounceCollider;
use crate::ball::Ball;
use crate::components::*;
use avian2d::prelude::*;
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, (spawn_paddles, spawn_gutters))
        .add_systems(FixedUpdate, (move_paddles, handle_player_input, move_ai));
}

#[derive(Component)]
#[require(
    PongPosition,
    PongVelocity,
    PongCollider = PongCollider(PADDLE_SHAPE)
)]
pub struct Paddle;

#[derive(Component)]
#[require(RigidBody::Static, BounceCollider)]
pub struct Gutter;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Ai;

const PADDLE_WIDTH: f32 = 50.0;
const PADDLE_HEIGHT: f32 = 300.0;
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
        Paddle,
        Mesh2d(mesh.clone()),
        MeshMaterial2d(material.clone()),
        PongPosition(player_position),
    ));

    let ai_position = Vec2::new(half_window_size.x - padding, 0.);

    commands.spawn((
        Ai,
        Paddle,
        Mesh2d(mesh.clone()),
        MeshMaterial2d(material.clone()),
        PongPosition(ai_position),
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
        BounceCollider(1.2),
        Position::from_xy(0.0, half_window),
        Collider::half_space(Vec2::NEG_Y),
        Mesh2d(mesh.clone()),
        MeshMaterial2d(material.clone()),
    ));

    commands.spawn((
        Gutter,
        BounceCollider(1.2),
        Position::from_xy(0.0, -half_window),
        Collider::half_space(Vec2::Y),
        Mesh2d(mesh.clone()),
        MeshMaterial2d(material.clone()),
    ));
}

const PADDLE_SPEED: f32 = 5.;

fn handle_player_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut paddle_velocity: Single<&mut PongVelocity, With<Player>>,
) {
    if keyboard_input.pressed(KeyCode::ArrowUp) {
        paddle_velocity.0.y = PADDLE_SPEED;
    } else if keyboard_input.pressed(KeyCode::ArrowDown) {
        paddle_velocity.0.y = -PADDLE_SPEED;
    } else {
        paddle_velocity.0.y = 0.;
    }
}

fn move_paddles(mut paddles: Query<(&mut PongPosition, &PongVelocity), With<Paddle>>) {
    for (mut position, velocity) in &mut paddles {
        position.0 += velocity.0;
    }
}

fn move_ai(
    ai: Single<(&mut PongVelocity, &PongPosition), With<Ai>>,
    ball: Single<&PongPosition, With<Ball>>,
) {
    let (mut velocity, position) = ai.into_inner();
    let a_to_b = ball.0 - position.0;
    velocity.0.y = a_to_b.y.signum() * PADDLE_SPEED;
}
