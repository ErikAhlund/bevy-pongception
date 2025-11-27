use crate::score::Scored;
use avian2d::prelude::*;
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_ball)
        .add_systems(FixedUpdate, limit_speed)
        .add_observer(reset_ball);
}

#[derive(Component)]
#[require(
    RigidBody::Dynamic,
    LinearVelocity(Vec2::new(MIN_SPEED, MIN_SPEED/2.0)),
    GravityScale(0.0),
    Collider::circle(BALL_SIZE),
    Restitution::with_combine_rule(&Restitution::new(1.2), CoefficientCombine::Max),
    Friction::with_combine_rule(&Friction::new(0.0), CoefficientCombine::Min),
    SweptCcd::default(),
    TransformInterpolation
)]
pub struct Ball;

const BALL_SIZE: f32 = 30.0;
const MIN_SPEED: f32 = 500.0;
const MAX_SPEED: f32 = 2000.0;

const BALL_SHAPE: Circle = Circle::new(BALL_SIZE);
const BALL_COLOR: Color = Color::srgb(1.0, 0., 0.);

fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mesh = meshes.add(BALL_SHAPE);
    let material = materials.add(BALL_COLOR);
    commands.spawn((Ball, Mesh2d(mesh), MeshMaterial2d(material)));
}

fn reset_ball(_event: On<Scored>, ball: Single<(&mut Position, &mut LinearVelocity), With<Ball>>) {
    let (mut ball_position, mut ball_velocity) = ball.into_inner();
    ball_position.0 = Vec2::ZERO;
    ball_velocity.0 = Vec2::new(MIN_SPEED, 0.);
}

fn limit_speed(ball_query: Query<&mut LinearVelocity, With<Ball>>) {
    for mut ball_velocity in ball_query {
        let speed = ball_velocity.length();
        println!("Speed is {}", speed);
        if speed > MAX_SPEED {
            ball_velocity.0 = ball_velocity.0.normalize_or_zero() * MAX_SPEED;
        }
    }
}
