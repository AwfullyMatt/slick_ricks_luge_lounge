use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;

use crate::{
    GameState, LugeState, Resolution,
    actions::GameAction,
    loading::{FontAssets, SpriteAssets},
    player::{Player, PlayerStats},
    ui::{ButtonColors, UiColor},
};

pub struct LugePlugin;

impl Plugin for LugePlugin {
    fn name(&self) -> &str {
        "Luge Plugin"
    }

    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Playing),
            (
                spawn_luigee,
                spawn_lanes,
                spawn_slick_ui,
                spawn_luigee_ui,
                update_lanes,
                set_input_cooldown,
            ),
        )
        .add_systems(
            Update,
            (
                consume_stale_input.run_if(resource_exists::<InputCooldown>),
                advance_dialogue,
                launch_button_handler,
            )
                .chain()
                .run_if(in_state(LugeState::Loadout)),
        )
        .add_systems(
            Update,
            (move_luigee, update_luigee_sprite, scroll_lanes).run_if(in_state(LugeState::Launched)),
        )
        .insert_resource(Lanes::default())
        .insert_resource(PlayerLane::default())
        .insert_resource(ScrollSpeed::default())
        .insert_resource(DialogueState::default())
        .insert_resource(RickLines::init());
    }
}

// marker components
#[derive(Component)]
struct LuigeeSprite;

#[derive(Component)]
struct LaneSprite;

#[derive(Component)]
struct RickDialogue;

#[derive(Component)]
struct LaunchButton;

fn spawn_luigee(mut commands: Commands, resolution: Res<Resolution>, sprites: Res<SpriteAssets>) {
    let y = -(resolution.vec2().y / 3.0);

    commands.spawn((
        Player,
        Player::default_input_map(),
        Sprite::from_image(sprites.luigee.clone()),
        Transform {
            translation: Vec3::new(0.0, y, 0.0),
            scale: Vec3::splat(resolution.scale()),
            ..default()
        },
        LuigeeSprite,
        DespawnOnExit(GameState::Playing),
    ));
}

fn spawn_lanes(mut commands: Commands, sprites: Res<SpriteAssets>, resolution: Res<Resolution>) {
    commands.spawn((
        DespawnOnExit(GameState::Playing),
        Sprite::from_image(sprites.lanes.clone()),
        Transform {
            translation: Vec3::new(0.0, 0.0, -1.0),
            scale: Vec3::splat(resolution.scale()),
            ..default()
        },
        LaneSprite,
    ));

    commands.spawn((
        DespawnOnExit(GameState::Playing),
        Sprite::from_image(sprites.lanes.clone()),
        Transform {
            translation: Vec3::new(0.0, 360.0 * resolution.scale(), -1.0),
            scale: Vec3::splat(resolution.scale()),
            ..default()
        },
        LaneSprite,
    ));
}

#[derive(Resource, Deref, DerefMut, Copy, Clone)]
pub struct ScrollSpeed(pub f32);

impl Default for ScrollSpeed {
    fn default() -> Self {
        Self(1000.0)
    }
}

#[derive(Resource, Copy, Clone, Default, Deref, DerefMut)]
pub struct PlayerLane(pub LaneLocation);

#[derive(Resource, Default)]
pub struct Lanes {
    left: Lane,
    center: Lane,
    right: Lane,
}

impl Lanes {
    fn init(resolution: Res<Resolution>) -> Self {
        let transform = resolution.calculate_lanes();
        Self {
            left: Lane { x: transform.0 },
            center: Lane { x: 0.0 },
            right: Lane { x: transform.1 },
        }
    }

    fn x_for(&self, lane: LaneLocation) -> f32 {
        use LaneLocation::*;
        match lane {
            Left => self.left.x,
            Center => self.center.x,
            Right => self.right.x,
        }
    }
}

#[derive(Default)]
pub struct Lane {
    pub x: f32,
}

#[derive(Default, Copy, Clone, Debug)]
pub enum LaneLocation {
    Left,
    #[default]
    Center,
    Right,
}

impl LaneLocation {
    pub fn shift_left(&self) -> Self {
        use LaneLocation::*;
        match self {
            Left => Left,
            Center => Left,
            Right => Center,
        }
    }

    pub fn shift_right(&self) -> Self {
        use LaneLocation::*;
        match self {
            Left => Center,
            Center => Right,
            Right => Right,
        }
    }
}

fn update_lanes(mut lanes: ResMut<Lanes>, resolution: Res<Resolution>) {
    *lanes = Lanes::init(resolution);
}

fn move_luigee(
    mut player_lane: ResMut<PlayerLane>,
    action_state: Single<&ActionState<GameAction>, With<Player>>,
) {
    if action_state.just_pressed(&GameAction::Left) {
        info!("Luge Action Left");
        **player_lane = player_lane.shift_left();
    }

    if action_state.just_pressed(&GameAction::Right) {
        info!("Luge Action Right");
        **player_lane = player_lane.shift_right();
    }
}

fn update_luigee_sprite(
    player_lane: Res<PlayerLane>,
    lanes: Res<Lanes>,
    mut transform: Single<&mut Transform, With<LuigeeSprite>>,
) {
    if player_lane.is_changed() {
        transform.translation.x = lanes.x_for(**player_lane);
        info!("Moved to {:?}", **player_lane);
    };
}

fn scroll_lanes(
    time: Res<Time>,
    player_stats: Res<PlayerStats>,
    scroll_speed: Res<ScrollSpeed>,
    resolution: Res<Resolution>,
    mut query_lane_sprites: Query<&mut Transform, With<LaneSprite>>,
) {
    let speed = **scroll_speed * (player_stats.speed as f32 / 10.0);
    let delta = speed * time.delta_secs();
    for mut transform in query_lane_sprites.iter_mut() {
        transform.translation.y -= delta;

        // screen wrap
        let y = resolution.vec2().y;
        if transform.translation.y <= -y {
            transform.translation.y += y * 2.0;
        }
    }
}

fn spawn_slick_ui(
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
                });
        });
}

fn spawn_luigee_ui(
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
                            LaunchButton,
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

fn launch_button_handler(
    mut next_luge_state: ResMut<NextState<LugeState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &ButtonColors),
        (Changed<Interaction>, With<LaunchButton>),
    >,
) {
    for (interaction, mut color, button_colors) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                next_luge_state.set(LugeState::Launched);
            }
            Interaction::Hovered => {
                *color = button_colors.hovered.into();
            }
            Interaction::None => {
                *color = button_colors.normal.into();
            }
        }
    }
}

#[derive(Resource)]
pub(crate) struct DialogueState {
    pub(crate) current_scene: SceneId,
    pub(crate) line_index: usize,
    pub(crate) waiting_for_input: bool,
}

impl Default for DialogueState {
    fn default() -> Self {
        Self {
            current_scene: SceneId::Intro,
            line_index: 0,
            waiting_for_input: true,
        }
    }
}

#[derive(Resource)]
struct RickLines(Vec<Scene>);

impl RickLines {
    fn init() -> Self {
        Self(vec![
            Scene {
                id: SceneId::Intro,
                lines: vec![
                    "Heya chump--errr, champ. Heh heh. \
                    Welcome ta Slick Rick's Luge Lounge. \
                    Da numbah one luge lounge in da lesser tri-state region."
                        .to_string(),
                    "Take a seat. Bob a sled. Spend some moolah. \
                    Su money es mi money, amigo. Capice?"
                        .to_string(),
                ],
                completed: false,
            },
            Scene {
                id: SceneId::Shop,
                lines: vec![],
                completed: false,
            },
        ])
    }

    fn get_scene(&self, id: SceneId) -> Option<&Scene> {
        self.0.iter().find(|s| s.id == id)
    }

    fn get_scene_mut(&mut self, id: SceneId) -> Option<&mut Scene> {
        self.0.iter_mut().find(|s| s.id == id)
    }

    fn get_line(&self, id: SceneId, index: usize) -> Option<&str> {
        self.get_scene(id)?.lines.get(index).map(|s| s.as_str())
    }
}

struct Scene {
    id: SceneId,
    lines: Vec<String>,
    completed: bool,
}

#[derive(PartialEq, Copy, Clone)]
pub(crate) enum SceneId {
    Intro,
    Shop,
}

impl SceneId {
    fn next(self) -> Option<Self> {
        match self {
            Self::Intro => Some(Self::Shop),
            Self::Shop => None,
        }
    }
}

fn advance_dialogue(
    mut dialogue_state: ResMut<DialogueState>,
    mut rick_lines: ResMut<RickLines>,
    mut rick_text: Single<&mut Text, With<RickDialogue>>,
    action_state: Single<&ActionState<GameAction>, With<Player>>,
) {
    if !dialogue_state.waiting_for_input {
        return;
    }

    if action_state.just_pressed(&GameAction::Continue) {
        dialogue_state.line_index += 1;

        if let Some(line) =
            rick_lines.get_line(dialogue_state.current_scene, dialogue_state.line_index)
        {
            rick_text.0 = line.to_string();
        } else {
            // Mark current scene completed
            if let Some(scene) = rick_lines.get_scene_mut(dialogue_state.current_scene) {
                scene.completed = true;
            }

            // Advance to next scene or finish dialogue
            if let Some(next_scene) = dialogue_state.current_scene.next() {
                dialogue_state.current_scene = next_scene;
                dialogue_state.line_index = 0;
                if let Some(line) = rick_lines.get_line(next_scene, 0) {
                    rick_text.0 = line.to_string();
                }
            } else {
                dialogue_state.waiting_for_input = false;
                rick_text.0 = String::new();
            }
        }
    }
}

#[derive(Resource)]
struct InputCooldown;

fn set_input_cooldown(mut commands: Commands) {
    commands.insert_resource(InputCooldown);
}

// disables and re-enables the action to prevent leak
// when clicking main menu play button
fn consume_stale_input(
    mut commands: Commands,
    mouse: Res<ButtonInput<MouseButton>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut action_state: Query<&mut ActionState<GameAction>>,
) {
    if mouse.pressed(MouseButton::Left) || keyboard.pressed(KeyCode::Space) {
        for mut state in &mut action_state {
            if !state.action_disabled(&GameAction::Continue) {
                state.disable_action(&GameAction::Continue);
            }
        }
        return;
    }

    for mut state in &mut action_state {
        state.enable_action(&GameAction::Continue);
    }
    commands.remove_resource::<InputCooldown>();
}
