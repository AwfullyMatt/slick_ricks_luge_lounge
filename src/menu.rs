use crate::loading::{SpriteAssets, TextureAssets};
use crate::ui::{ButtonColors, ChangeState, OpenLink, UiColor, font_size_for};
use crate::{GameState, Resolution};
use bevy::prelude::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), setup_menu);
    }
}

fn setup_menu(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    sprites: Res<SpriteAssets>,
    resolution: Res<Resolution>,
) {
    let s = resolution.ui_scale();

    // main button dimensions
    let btn_w = 140.0 * s;
    let btn_h = 50.0 * s;
    let border = 4.0 * s;
    let pad_x = 16.0 * s;
    let pad_y = 8.0 * s;

    // footer dimensions
    let footer_w = 170.0 * s;
    let footer_h = 50.0 * s;
    let footer_pad = 5.0 * s;
    let icon_size = 32.0 * s;
    let bottom_offset = 5.0 * s;

    // font sizes derived from button dimensions + text length
    let play_font = font_size_for(btn_w, btn_h, "Play");
    let settings_font = font_size_for(btn_w, btn_h, "Settings");
    let footer_text_w = footer_w - icon_size;
    let footer_font = font_size_for(footer_text_w, footer_h, "Made with Bevy");

    // spawn title screen
    commands.spawn((
        ImageNode::new(sprites.title.clone()),
        Node {
            width: Val::Percent(100.0),
            height: Val::Auto,
            ..default()
        },
        DespawnOnExit(GameState::Menu),
    ));
    //spawn buttons
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            DespawnOnExit(GameState::Menu),
        ))
        .with_children(|children| {
            let button_colors = ButtonColors::default();
            children
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(btn_w),
                        height: Val::Px(btn_h),
                        border: UiRect::all(Val::Px(border)),
                        padding: UiRect::axes(Val::Px(pad_x), Val::Px(pad_y)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    BorderColor::all(UiColor::Darkest.color()),
                    BackgroundColor(button_colors.normal),
                    button_colors.clone(),
                    ChangeState(GameState::Playing),
                ))
                .with_child((
                    Text::new("Play"),
                    TextFont {
                        font_size: play_font,
                        ..default()
                    },
                    TextColor(UiColor::Darkest.color()),
                ));
            children
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(btn_w),
                        height: Val::Px(btn_h),
                        border: UiRect::all(Val::Px(border)),
                        padding: UiRect::axes(Val::Px(pad_x), Val::Px(pad_y)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    BorderColor::all(UiColor::Darkest.color()),
                    BackgroundColor(button_colors.normal),
                    button_colors,
                    ChangeState(GameState::Settings),
                ))
                .with_child((
                    Text::new("Settings"),
                    TextFont {
                        font_size: settings_font,
                        ..default()
                    },
                    TextColor(UiColor::Darkest.color()),
                ));
        });
    commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceAround,
                bottom: Val::Px(bottom_offset),
                width: Val::Percent(100.),
                position_type: PositionType::Absolute,
                ..default()
            },
            DespawnOnExit(GameState::Menu),
        ))
        .with_children(|children| {
            children
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(footer_w),
                        height: Val::Px(footer_h),
                        justify_content: JustifyContent::SpaceAround,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(footer_pad)),
                        ..Default::default()
                    },
                    BackgroundColor(Color::NONE),
                    ButtonColors {
                        normal: Color::NONE,
                        ..default()
                    },
                    OpenLink("https://bevyengine.org"),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Made with Bevy"),
                        TextFont {
                            font_size: footer_font,
                            ..default()
                        },
                        TextColor(Color::linear_rgb(0.9, 0.9, 0.9)),
                    ));
                    parent.spawn((
                        ImageNode {
                            image: textures.bevy.clone(),
                            ..default()
                        },
                        Node {
                            width: Val::Px(icon_size),
                            ..default()
                        },
                    ));
                });
            children
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(footer_w),
                        height: Val::Px(footer_h),
                        justify_content: JustifyContent::SpaceAround,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(footer_pad)),
                        ..default()
                    },
                    BackgroundColor(Color::NONE),
                    ButtonColors {
                        normal: Color::NONE,
                        hovered: Color::linear_rgb(0.25, 0.25, 0.25),
                    },
                    OpenLink("https://github.com/NiklasEi/bevy_game_template"),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Open source"),
                        TextFont {
                            font_size: footer_font,
                            ..default()
                        },
                        TextColor(Color::linear_rgb(0.9, 0.9, 0.9)),
                    ));
                    parent.spawn((
                        ImageNode::new(textures.github.clone()),
                        Node {
                            width: Val::Px(icon_size),
                            ..default()
                        },
                    ));
                });
        });
}
