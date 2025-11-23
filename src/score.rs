use crate::ball::Ball;
use crate::components::{PongCollider, PongPosition};
use crate::paddle::{Ai, Player};
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.insert_resource(Score { player: 0, ai: 0 })
        .add_systems(Startup, spawn_scoreboard)
        .add_systems(FixedUpdate, (detect_goal, update_scoreboard))
        .add_observer(update_score);
}

#[derive(Resource)]
struct Score {
    player: u32,
    ai: u32,
}

#[derive(EntityEvent)]
pub struct Scored {
    #[event_target]
    pub scorer: Entity,
}

#[derive(Component)]
pub struct PlayerScore;

#[derive(Component)]
pub struct AiScore;

fn detect_goal(
    ball: Single<(&PongPosition, &PongCollider), With<Ball>>,
    player: Single<Entity, (With<Player>, Without<Ai>)>,
    ai: Single<Entity, (With<Ai>, Without<Player>)>,
    window: Single<&Window>,
    mut commands: Commands,
) {
    let (ball_position, ball_collider) = ball.into_inner();
    let half_window_size = window.resolution.size() / 2.;

    if ball_position.0.x - ball_collider.half_size().x > half_window_size.x {
        commands.trigger(Scored { scorer: *player });
    }

    if ball_position.0.x + ball_collider.half_size().x < -half_window_size.x {
        commands.trigger(Scored { scorer: *ai });
    }
}

fn update_scoreboard(
    mut player_score: Single<&mut Text, (With<PlayerScore>, Without<AiScore>)>,
    mut ai_score: Single<&mut Text, (With<AiScore>, Without<PlayerScore>)>,
    score: Res<Score>,
) {
    if score.is_changed() {
        player_score.0 = score.player.to_string();
        ai_score.0 = score.ai.to_string();
    }
}

fn spawn_scoreboard(mut commands: Commands) {
    let container = Node {
        width: percent(100.0),
        height: percent(100.0),
        justify_content: JustifyContent::Center,
        ..default()
    };

    let header = Node {
        width: px(200.),
        height: px(100.),
        ..default()
    };

    // The players score on the left hand side
    let player_score = (
        PlayerScore,
        Text::new("0"),
        TextFont::from_font_size(72.0),
        TextColor(Color::WHITE),
        TextLayout::new_with_justify(Justify::Center),
        Node {
            position_type: PositionType::Absolute,
            top: px(5.),
            left: px(0.),
            ..default()
        },
    );

    // The AI score on the right hand side
    let ai_score = (
        AiScore,
        Text::new("0"),
        TextFont::from_font_size(72.0),
        TextColor(Color::WHITE),
        TextLayout::new_with_justify(Justify::Center),
        Node {
            position_type: PositionType::Absolute,
            top: px(5.),
            right: px(0.),
            ..default()
        },
    );

    commands.spawn((
        container,
        children![(header, children![player_score, ai_score])],
    ));
}

fn update_score(
    event: On<Scored>,
    mut score: ResMut<Score>,
    is_ai: Query<&Ai>,
    is_player: Query<&Player>,
) {
    if is_ai.get(event.scorer).is_ok() {
        score.ai += 1;
        info!("AI scored! {} - {}", score.player, score.ai);
    }

    if is_player.get(event.scorer).is_ok() {
        score.player += 1;
        info!("Player scored! {} - {}", score.player, score.ai);
    }
}
