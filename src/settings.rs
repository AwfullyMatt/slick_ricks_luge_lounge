use bevy::prelude::*;

use crate::{
    GameState, Resolution,
    loading::FontAssets,
    ui::{ButtonColors, ChangeState, UiColor},
};

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn name(&self) -> &str {
        "Settings Plugin"
    }

    fn build(&self, app: &mut App) {
        app.init_resource::<Resolution>();
        app.add_systems(Startup, set_window_resolution)
            .add_systems(OnEnter(GameState::Settings), spawn_settings_menu)
            .add_systems(
                Update,
                (toggle_dropdown, select_option).run_if(in_state(GameState::Settings)),
            );
    }
}

// marker components
#[derive(Component)]
pub struct DropdownHead;

#[derive(Component)]
pub struct DropdownLabel;

#[derive(Component)]
pub struct DropdownPanel;

#[derive(Component)]
pub struct DropdownChoice(pub Resolution);

fn set_window_resolution(mut window: Single<&mut Window>, resolution: Res<Resolution>) {
    let res = resolution.vec2();
    window.resolution.set(res.x, res.y);
}

fn spawn_settings_menu(
    mut commands: Commands,
    resolution: Res<Resolution>,
    fonts: Res<FontAssets>,
) {
    let s = resolution.ui_scale();

    // button dimensions (auto-height — text determines button size)
    let btn_w = 140.0 * s;
    let border = 4.0 * s;
    let pad_x = 16.0 * s;
    let pad_y = 8.0 * s;

    // layout gaps
    let row_gap = 20.0 * s;
    let col_gap = 16.0 * s;

    // standalone text sizes scale linearly
    let title_font = 48.0 * s;
    let label_font = 24.0 * s;

    // button font proportional to button width
    let btn_font = btn_w / 7.0;

    // dropdown panel offset = head border + padding + font + border
    let dropdown_top = border * 2.0 + pad_y + btn_font;

    commands
        .spawn((
            DespawnOnExit(GameState::Settings),
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(row_gap),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Settings"),
                TextFont {
                    font: fonts.tiny5.clone(),
                    font_size: title_font,
                    ..default()
                },
                TextColor(UiColor::Darkest.color()),
            ));

            let button_colors = ButtonColors::default();
            parent
                .spawn(Node {
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(col_gap),
                    ..default()
                })
                .with_children(|row| {
                    row.spawn((
                        Text::new("Resolution:"),
                        TextFont {
                            font: fonts.tiny5.clone(),
                            font_size: label_font,
                            ..default()
                        },
                        TextColor(UiColor::Darkest.color()),
                    ));

                    row.spawn(Node {
                        flex_direction: FlexDirection::Column,
                        ..default()
                    })
                    .with_children(|dropdown| {
                        dropdown
                            .spawn((
                                DropdownHead,
                                Button,
                                Node {
                                    width: Val::Px(btn_w),
                                    border: UiRect::all(Val::Px(border)),
                                    padding: UiRect::axes(Val::Px(pad_x), Val::Px(pad_y)),
                                    justify_content: JustifyContent::Center,
                                    ..default()
                                },
                                BackgroundColor(UiColor::Lighter.color()),
                                BorderColor::all(UiColor::Darkest.color()),
                                button_colors.clone(),
                            ))
                            .with_child((
                                DropdownLabel,
                                Text::new(resolution.label()),
                                TextFont {
                                    font: fonts.tiny5.clone(),
                                    font_size: btn_font,
                                    ..default()
                                },
                                TextColor(UiColor::Darkest.color()),
                            ));

                        dropdown
                            .spawn((
                                DropdownPanel,
                                Node {
                                    display: Display::None,
                                    border: UiRect::all(Val::Px(border)),
                                    padding: UiRect::axes(Val::Px(pad_x), Val::Px(pad_y)),
                                    flex_direction: FlexDirection::Column,
                                    position_type: PositionType::Absolute,
                                    top: Val::Px(dropdown_top),
                                    width: Val::Px(btn_w),
                                    ..default()
                                },
                                BackgroundColor(UiColor::Lightest.color()),
                                BorderColor::all(UiColor::Darkest.color()),
                                button_colors.clone(),
                                GlobalZIndex(10),
                            ))
                            .with_children(|panel| {
                                for res in Resolution::RESOLUTIONS {
                                    panel
                                        .spawn((
                                            DropdownChoice(res),
                                            Button,
                                            Node {
                                                width: Val::Percent(100.0),
                                                padding: UiRect::axes(
                                                    Val::Px(pad_x),
                                                    Val::Px(pad_y),
                                                ),
                                                justify_content: JustifyContent::Center,
                                                ..default()
                                            },
                                            BackgroundColor(UiColor::Light.color()),
                                            BorderColor::all(UiColor::Darkest.color()),
                                            button_colors.clone(),
                                        ))
                                        .with_child((
                                            Text::new(res.label()),
                                            TextFont {
                                                font: fonts.tiny5.clone(),
                                                font_size: btn_font,
                                                ..default()
                                            },
                                            TextColor(UiColor::Darkest.color()),
                                        ));
                                }
                            });
                    });
                });
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(btn_w),
                        border: UiRect::all(Val::Px(border)),
                        padding: UiRect::axes(Val::Px(pad_x), Val::Px(pad_y)),
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    BackgroundColor(UiColor::Lighter.color()),
                    BorderColor::all(UiColor::Darkest.color()),
                    ChangeState(GameState::Menu),
                    button_colors.clone(),
                ))
                .with_child((
                    Text::new("Back"),
                    TextFont {
                        font: fonts.tiny5.clone(),
                        font_size: btn_font,
                        ..default()
                    },
                    TextColor(UiColor::Darkest.color()),
                ));
        });
}

fn toggle_dropdown(
    head_query: Query<&Interaction, (Changed<Interaction>, With<DropdownHead>)>,
    mut panel: Single<&mut Node, With<DropdownPanel>>,
) {
    for interaction in &head_query {
        if *interaction == Interaction::Pressed {
            panel.display = match panel.display {
                Display::None => Display::Flex,
                _ => Display::None,
            }
        }
    }
}

fn select_option(
    choice_query: Query<(&Interaction, &DropdownChoice), Changed<Interaction>>,
    mut label: Single<&mut Text, With<DropdownLabel>>,
    mut panel: Single<&mut Node, With<DropdownPanel>>,
    mut resolution: ResMut<Resolution>,
    mut window: Single<&mut Window>,
) {
    for (interaction, choice) in &choice_query {
        if *interaction == Interaction::Pressed {
            *resolution = choice.0;
            let res = choice.0.vec2();
            window.resolution.set(res.x, res.y);
            label.0 = choice.0.label().to_string();
            panel.display = Display::None;
        }
    }
}
