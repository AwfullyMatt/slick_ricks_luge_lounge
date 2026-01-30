use bevy::prelude::*;

use crate::{
    GameState,
    menu::{ButtonColors, ChangeState},
    ui::UiColor,
};

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn name(&self) -> &str {
        "Settings Plugin"
    }

    fn build(&self, app: &mut App) {
        app.insert_resource(Settings::default());
        app.add_systems(Startup, set_window_resolution)
            .add_systems(OnEnter(GameState::Settings), spawn_settings_menu)
            .add_systems(
                Update,
                (toggle_dropdown, select_option).run_if(in_state(GameState::Settings)),
            )
            .add_systems(OnExit(GameState::Settings), cleanup_settings);
    }
}

// marker components
#[derive(Component)]
pub struct SettingsCleanup;

#[derive(Component)]
pub struct DropdownHead;

#[derive(Component)]
pub struct DropdownLabel;

#[derive(Component)]
pub struct DropdownPanel;

#[derive(Component)]
pub struct DropdownChoice(pub Resolution);

#[derive(Resource, Default)]
pub struct Settings {
    pub resolution: Resolution,
}

#[derive(Default, Clone, Copy)]
pub enum Resolution {
    Sd,
    #[default]
    Hd,
    Qhd,
    Uhd,
}

impl Resolution {
    const RESOLUTIONS: [Self; 4] = [Self::Sd, Self::Hd, Self::Qhd, Self::Uhd];

    pub fn vec2(&self) -> Vec2 {
        use Resolution::*;
        match self {
            Sd => Vec2::new(1280.0, 720.0),
            Hd => Vec2::new(1920.0, 1080.0),
            Qhd => Vec2::new(2560.0, 1440.0),
            Uhd => Vec2::new(3840.0, 2160.0),
        }
    }

    pub fn label(&self) -> &'static str {
        use Resolution::*;
        match self {
            Sd => "720p",
            Hd => "1080p",
            Qhd => "1440p",
            Uhd => "4k",
        }
    }

    pub fn calculate_lanes(&self) -> (f32, f32) {
        match self {
            // lane is 200 wide, that's x2 scale
            // at 720p, 200 / 3 = 66.6
            // 66 x 2 = 132
            Resolution::Sd => (-132.0, 132.0),
            // times 3 at 1080p
            Resolution::Hd => (-198.0, 198.0),
            // times 4 at 1440p
            Resolution::Qhd => (-264.0, 264.0),
            // times 5 at 4k
            Resolution::Uhd => (-330.0, 330.0),
            // should be unreachable
            _ => (0.0, 0.0),
        }
    }

    pub fn scale(&self) -> f32 {
        match self {
            Resolution::Sd => 2.0,
            Resolution::Hd => 3.0,
            Resolution::Qhd => 4.0,
            Resolution::Uhd => 5.0,
            _ => 0.0,
        }
    }
}

fn set_window_resolution(mut window: Single<&mut Window>, settings: Res<Settings>) {
    let res = settings.resolution.vec2();
    // for now just setting to hd due to default
    window.resolution.set(res.x, res.y);
}

fn spawn_settings_menu(mut commands: Commands, settings: Res<Settings>) {
    commands
        .spawn((
            SettingsCleanup,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(20.0),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Settings"),
                TextFont {
                    font_size: 48.0,
                    ..default()
                },
            ));

            let button_colors = ButtonColors::default();
            parent
                .spawn(Node {
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(16.0),
                    ..default()
                })
                .with_children(|row| {
                    row.spawn((
                        Text::new("Resolution:"),
                        TextFont {
                            font_size: 24.0,
                            ..default()
                        },
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
                                    width: Val::Px(140.0),
                                    border: UiRect::all(Val::Px(4.0)),
                                    padding: UiRect::axes(Val::Px(16.0), Val::Px(8.0)),
                                    justify_content: JustifyContent::Center,
                                    ..default()
                                },
                                BackgroundColor(UiColor::Lighter.linear_rgb()),
                                BorderColor::all(UiColor::Darkest.linear_rgb()),
                                button_colors.clone(),
                            ))
                            .with_child((
                                DropdownLabel,
                                Text::new(settings.resolution.label()),
                                TextFont {
                                    font_size: 20.0,
                                    ..default()
                                },
                            ));

                        dropdown
                            .spawn((
                                DropdownPanel,
                                Node {
                                    display: Display::None,
                                    border: UiRect::all(Val::Px(4.0)),
                                    padding: UiRect::axes(Val::Px(16.0), Val::Px(8.0)),
                                    flex_direction: FlexDirection::Column,
                                    position_type: PositionType::Absolute,
                                    top: Val::Px(36.0),
                                    width: Val::Px(140.0),
                                    ..default()
                                },
                                BackgroundColor(UiColor::Lightest.linear_rgb()),
                                BorderColor::all(UiColor::Darkest.linear_rgb()),
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
                                                padding: UiRect::axes(Val::Px(16.0), Val::Px(8.0)),
                                                justify_content: JustifyContent::Center,
                                                ..default()
                                            },
                                            BackgroundColor(UiColor::Light.linear_rgb()),
                                            BorderColor::all(UiColor::Darkest.linear_rgb()),
                                            button_colors.clone(),
                                        ))
                                        .with_child((
                                            Text::new(res.label()),
                                            TextFont {
                                                font_size: 20.0,
                                                ..default()
                                            },
                                        ));
                                }
                            });
                    });

                    row.spawn((
                        Button,
                        Node {
                            width: Val::Px(140.0),
                            border: UiRect::all(Val::Px(4.0)),
                            padding: UiRect::axes(Val::Px(16.0), Val::Px(8.0)),
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        BackgroundColor(UiColor::Lighter.linear_rgb()),
                        BorderColor::all(UiColor::Darkest.linear_rgb()),
                        ChangeState(GameState::Menu),
                        button_colors.clone(),
                    ))
                    .with_child((
                        Text::new("Back"),
                        TextFont {
                            font_size: 20.0,
                            ..default()
                        },
                    ));
                });
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
    mut settings: ResMut<Settings>,
    mut window: Single<&mut Window>,
) {
    for (interaction, choice) in &choice_query {
        if *interaction == Interaction::Pressed {
            settings.resolution = choice.0;
            let res = choice.0.vec2();
            window.resolution.set(res.x, res.y);
            label.0 = choice.0.label().to_string();
            panel.display = Display::None;
        }
    }
}

fn cleanup_settings(mut commands: Commands, cleanup_query: Query<Entity, With<SettingsCleanup>>) {
    for entity in cleanup_query.iter() {
        commands.entity(entity).despawn();
    }
}
