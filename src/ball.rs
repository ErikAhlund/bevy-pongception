use crate::components::*;
use crate::score::Scored;
use avian2d::prelude::*;
use bevy::math::bounding::Aabb2d;
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_ball)
        .add_systems(FixedUpdate, (move_ball, handle_collisions))
        .add_observer(reset_ball);
}

#[derive(Component)]
#[require(
    PongPosition,
    PongVelocity(Vec2::new(0.0, MIN_SPEED)),
    PongCollider(Rectangle::new(BALL_SIZE, BALL_SIZE)),
    RigidBody::Kinematic,
    Collider::circle(BALL_SIZE)
)]
pub struct Ball;

const BALL_SIZE: f32 = 30.0;
const MIN_SPEED: f32 = 30.0;
const MAX_SPEED: f32 = 80.0;
const SPEED_MULT: f32 = 1.1;
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

fn move_ball(ball: Single<(&mut PongPosition, &PongVelocity), With<Ball>>) {
    let (mut position, velocity) = ball.into_inner();
    position.0 += velocity.0;
}

fn reset_ball(
    _event: On<Scored>,
    ball: Single<(&mut PongPosition, &mut PongVelocity), With<Ball>>,
) {
    let (mut ball_position, mut ball_velocity) = ball.into_inner();
    ball_position.0 = Vec2::ZERO;
    ball_velocity.0 = Vec2::new(MIN_SPEED, 0.);
}

fn limit_speed(velocity: &mut Vec2, max_speed: f32) {
    let speed = velocity.length();
    if speed > max_speed {
        *velocity = velocity.normalize_or_zero() * max_speed;
    }
}

fn handle_collisions(
    ball: Single<(&mut PongVelocity, &PongPosition, &PongCollider), With<Ball>>,
    other_things: Query<(&PongPosition, &PongCollider), Without<Ball>>,
) {
    let (mut ball_velocity, ball_position, ball_collider) = ball.into_inner();

    for (other_position, other_collider) in &other_things {
        if let Some(collision) = crate::collisions::collide_with_side(
            Aabb2d::new(ball_position.0, ball_collider.half_size()),
            Aabb2d::new(other_position.0, other_collider.half_size()),
        ) {
            match collision {
                PongCollision::Left => {
                    ball_velocity.0.x *= -SPEED_MULT;
                }
                PongCollision::Right => {
                    ball_velocity.0.x *= -SPEED_MULT;
                }
                PongCollision::Top => {
                    ball_velocity.0.y *= -SPEED_MULT;
                }
                PongCollision::Bottom => {
                    ball_velocity.0.y *= -SPEED_MULT;
                }
            }

            limit_speed(&mut ball_velocity.0, MAX_SPEED);
        }
    }
}
