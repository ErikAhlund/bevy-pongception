use super::collisions::*;
use crate::ball::Ball;
use avian2d::prelude::*;
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, (spawn_paddles, spawn_gutters))
        .add_systems(FixedUpdate, (move_ai));
}

const PADDLE_WIDTH: f32 = 50.0;
const PADDLE_HEIGHT: f32 = 300.0;

#[derive(Component)]
#[require(
    RigidBody::Dynamic,
    ShapeCaster::new(
        Collider::rectangle(PADDLE_WIDTH, PADDLE_HEIGHT),
        Vec2::ZERO, // Offset
        0.0,        // Rotation
        Dir2::Y     // Initial direction (system will update this)
    ),
    Collider::rectangle(PADDLE_WIDTH, PADDLE_HEIGHT),
    // 4. Our new Component
    LinearVelocity(Vec2::Y * 100.0)
)]
pub struct Paddle;

#[derive(Component)]
#[require(RigidBody::Static, BounceCollider)]
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
        Paddle,
        Mesh2d(mesh.clone()),
        MeshMaterial2d(material.clone()),
        Position(player_position),
    ));

    let ai_position = Vec2::new(half_window_size.x - padding, 0.);
    commands.spawn((
        Ai,
        Paddle,
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

const PADDLE_SPEED: f32 = 500.;
fn move_ai(
    ai: Single<(&mut LinearVelocity, &Position, &RigidBody), With<Ai>>,
    ball: Single<&Position, With<Ball>>,
) {
    let (mut velocity, position, body) = ai.into_inner();

    let a_to_b = ball.0 - position.0;
    velocity.0.y = PADDLE_SPEED;
}
