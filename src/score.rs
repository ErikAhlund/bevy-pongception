use crate::ball::Ball;
use crate::paddle::{Ai, Player};
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.insert_resource(Score {
        player1: 0,
        player2: 0,
    })
    .add_systems(Startup, spawn_scoreboard)
    .add_systems(FixedUpdate, (update_scoreboard))
    .add_observer(update_score);
}

#[derive(Resource)]
struct Score {
    player1: u32,
    player2: u32,
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

fn update_scoreboard(
    mut player_score: Single<&mut Text, (With<PlayerScore>, Without<AiScore>)>,
    mut ai_score: Single<&mut Text, (With<AiScore>, Without<PlayerScore>)>,
    score: Res<Score>,
) {
    if score.is_changed() {
        player_score.0 = score.player1.to_string();
        ai_score.0 = score.player2.to_string();
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
        score.player2 += 1;
        info!("AI scored! {} - {}", score.player1, score.player2);
    }

    if is_player.get(event.scorer).is_ok() {
        score.player1 += 1;
        info!("Player scored! {} - {}", score.player1, score.player2);
    }
}
