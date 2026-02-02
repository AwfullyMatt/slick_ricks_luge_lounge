use bevy::prelude::*;

use crate::{
    GameState, LugeState, Resolution,
    loading::{FontAssets, SpriteAssets},
    player::PlayerStats,
    ui::{ButtonColors, ChangeLugeState, UiColor},
};

use super::RunTimer;
use super::dialogue::{DialogueState, RickDialogue, RickLines};
use super::spawner::PlayerCoins;

#[derive(Component)]
pub(super) struct DialogueHint;

#[derive(Component)]
pub(super) struct RunTimerText;

#[derive(Component)]
pub(super) struct CoinCountText;

pub(super) fn spawn_slick_ui(
    mut commands: Commands,
    resolution: Res<Resolution>,
    sprites: Res<SpriteAssets>,
    fonts: Res<FontAssets>,
    dialogue_state: Res<DialogueState>,
    rick_lines: Res<RickLines>,
) {
    let s = resolution.ui_scale();
    let border = 8.0 * s;
    let initial_text = rick_lines
        .get_line(dialogue_state.current_scene, dialogue_state.line_index)
        .unwrap_or("");

    commands
        .spawn((
            Name::new("Slick UI Parent"),
            DespawnOnExit(GameState::Playing),
            Node {
                width: Val::Percent(25.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                border: UiRect::all(Val::Px(border)),
                ..default()
            },
            BackgroundColor(UiColor::Dark.color()),
            BorderColor::all(UiColor::Darkest.color()),
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Name::new("Slick Rick Container"),
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(35.0),
                        border: UiRect::all(Val::Px(border)),
                        padding: UiRect::axes(Val::Px(0.0), Val::Px(0.0)),
                        position_type: PositionType::Absolute,
                        top: Val::Px(0.0),
                        ..default()
                    },
                    BorderColor::all(UiColor::Darker.color()),
                ))
                .with_children(|img_parent| {
                    img_parent.spawn((
                        Name::new("Slick Rick"),
                        ImageNode::new(sprites.slick_rick.clone()),
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            ..default()
                        },
                    ));
                });
            parent
                .spawn((
                    Name::new("Rick Text Container"),
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(65.0),
                        border: UiRect::all(Val::Px(border)),
                        padding: UiRect::axes(Val::Px(border), Val::Px(border)),
                        position_type: PositionType::Absolute,
                        bottom: Val::Px(0.0),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::SpaceBetween,
                        ..default()
                    },
                    BorderColor::all(UiColor::Light.color()),
                ))
                .with_children(|text_parent| {
                    text_parent.spawn((
                        RickDialogue,
                        Text::new(initial_text),
                        TextFont {
                            font: fonts.tiny5.clone(),
                            font_size: 24.0 * s,
                            ..default()
                        },
                    ));
                    text_parent.spawn((
                        DialogueHint,
                        Text::new("[Space / Click] to continue"),
                        TextFont {
                            font: fonts.tiny5.clone(),
                            font_size: 16.0 * s,
                            ..default()
                        },
                        TextColor(UiColor::Lighter.color()),
                    ));
                });
        });
}

pub(super) fn spawn_luigee_ui(
    mut commands: Commands,
    resolution: Res<Resolution>,
    fonts: Res<FontAssets>,
    player_stats: Res<PlayerStats>,
) {
    let s = resolution.ui_scale();
    let border = 8.0 * s;

    commands
        .spawn((
            Name::new("Luigee UI Parent"),
            DespawnOnExit(GameState::Playing),
            Node {
                width: Val::Percent(25.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                right: Val::Px(0.0),
                border: UiRect::all(Val::Px(border)),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(UiColor::Dark.color()),
            BorderColor::all(UiColor::Darkest.color()),
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Name::new("Luigee Stats Container"),
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(65.0),
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(border)),
                        border: UiRect::all(Val::Px(border)),
                        ..default()
                    },
                    BorderColor::all(UiColor::Darker.color()),
                ))
                .with_children(|stats_parent| {
                    let font = fonts.tiny5.clone();
                    stats_parent.spawn((
                        Text::new("Luigee"),
                        TextFont {
                            font: font.clone(),
                            font_size: 36.0 * s,
                            ..default()
                        },
                    ));
                    let font_size = 36.0 * s;
                    let stats = [
                        format!("ATK: {}", player_stats.attack),
                        format!("DEF: {}", player_stats.defence),
                        format!("SPD: {}", player_stats.speed),
                        format!("LCK: {}", player_stats.luck),
                    ];
                    for stat in stats {
                        stats_parent.spawn((
                            Text::new(stat),
                            TextFont {
                                font: font.clone(),
                                font_size,
                                ..default()
                            },
                        ));
                    }
                    stats_parent.spawn((
                        RunTimerText,
                        Text::new("00:00.00"),
                        TextFont {
                            font: font.clone(),
                            font_size,
                            ..default()
                        },
                    ));
                    stats_parent.spawn((
                        CoinCountText,
                        Text::new("Coins: 0"),
                        TextFont {
                            font: font.clone(),
                            font_size,
                            ..default()
                        },
                    ));
                });

            parent
                .spawn((
                    Name::new("Launch Button Container"),
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(35.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border: UiRect::all(Val::Px(border)),
                        ..default()
                    },
                    BorderColor::all(UiColor::Darker.color()),
                ))
                .with_children(|btn_parent| {
                    btn_parent
                        .spawn((
                            Name::new("Launch Button"),
                            Button,
                            Visibility::Hidden,
                            ChangeLugeState(LugeState::Launched),
                            ButtonColors::default(),
                            Node {
                                padding: UiRect::axes(Val::Px(24.0 * s), Val::Px(12.0 * s)),
                                border: UiRect::all(Val::Px(4.0 * s)),
                                ..default()
                            },
                            BackgroundColor(UiColor::Light.color()),
                            BorderColor::all(UiColor::Darkest.color()),
                        ))
                        .with_children(|label| {
                            label.spawn((
                                Text::new("LAUNCH!"),
                                TextFont {
                                    font: fonts.tiny5.clone(),
                                    font_size: 24.0 * s,
                                    ..default()
                                },
                                TextColor(UiColor::Darkest.color()),
                            ));
                        });
                });
        });
}

pub(super) fn toggle_launch_button(
    dialogue_state: Res<DialogueState>,
    mut button: Single<&mut Visibility, With<ChangeLugeState>>,
    mut hint: Single<&mut Visibility, (With<DialogueHint>, Without<ChangeLugeState>)>,
) {
    if dialogue_state.waiting_for_input {
        **button = Visibility::Hidden;
        **hint = Visibility::Visible;
    } else {
        **button = Visibility::Visible;
        **hint = Visibility::Hidden;
    }
}

pub(super) fn update_run_timer_text(
    timer: Res<RunTimer>,
    mut query: Query<&mut Text, With<RunTimerText>>,
) {
    let elapsed = timer.0.elapsed_secs();
    let minutes = (elapsed / 60.0) as u32;
    let seconds = elapsed % 60.0;
    for mut text in &mut query {
        **text = format!("{:02}:{:05.2}", minutes, seconds);
    }
}

pub(super) fn update_coin_count_text(
    player_coins: Res<PlayerCoins>,
    mut query: Query<&mut Text, With<CoinCountText>>,
) {
    if player_coins.is_changed() {
        for mut text in &mut query {
            **text = format!("Coins: {}", **player_coins);
        }
    }
}
