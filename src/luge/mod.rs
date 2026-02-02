mod dialogue;
mod spawner;
mod ui;

use bevy::{prelude::*, time::Stopwatch};
use leafwing_input_manager::prelude::ActionState;

use crate::{
    GameState, LugeState, Resolution,
    actions::GameAction,
    loading::SpriteAssets,
    player::{Player, PlayerStats},
};

use dialogue::{DialogueState, RickLines};

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
                ui::spawn_slick_ui,
                ui::spawn_luigee_ui,
                update_lanes,
                set_input_cooldown,
                spawner::init_coin_atlas,
                spawner::reset_player_coins,
            ),
        )
        .add_systems(OnEnter(LugeState::Loadout), reset_luge)
        .add_systems(
            OnEnter(LugeState::Launched),
            (
                reset_run_timer,
                reset_scroll_speed,
                spawner::init_spawn_timer,
                spawner::spawn_initial_coin,
            ),
        )
        .add_systems(
            Update,
            (
                consume_stale_input.run_if(resource_exists::<InputCooldown>),
                dialogue::advance_dialogue,
                ui::toggle_launch_button,
            )
                .chain()
                .run_if(in_state(LugeState::Loadout)),
        )
        .add_systems(
            Update,
            (
                tick_run_timer,
                ui::update_run_timer_text,
                decelerate_luigee,
                move_luigee,
                update_luigee_sprite,
                scroll_lanes,
                (
                    spawner::spawn_coins,
                    spawner::scroll_occupants,
                    spawner::collect_coins,
                    spawner::despawn_offscreen,
                )
                    .chain(),
                ui::update_coin_count_text,
            )
                .run_if(in_state(LugeState::Launched)),
        )
        .add_systems(OnExit(LugeState::Launched), spawner::cleanup_spawner)
        .insert_resource(Lanes::default())
        .insert_resource(PlayerLane::default())
        .insert_resource(ScrollSpeed::default())
        .insert_resource(DialogueState::default())
        .insert_resource(RunTimer::default())
        .insert_resource(RickLines::init())
        .insert_resource(spawner::PlayerCoins::default());
    }
}

// marker components
#[derive(Component)]
pub(super) struct LuigeeSprite;

#[derive(Component)]
struct LaneSprite;

#[derive(Resource, Default, Deref, DerefMut)]
struct RunTimer(Stopwatch);

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

#[derive(Default, Copy, Clone, Debug, PartialEq)]
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

fn tick_run_timer(time: Res<Time>, mut timer: ResMut<RunTimer>) {
    timer.0.tick(time.delta());
}

fn reset_run_timer(mut timer: ResMut<RunTimer>) {
    timer.0.reset();
}

const BASE_DECELERATION: f32 = 50.0;

fn decelerate_luigee(
    time: Res<Time>,
    player_stats: Res<PlayerStats>,
    mut scroll_speed: ResMut<ScrollSpeed>,
    mut next_state: ResMut<NextState<LugeState>>,
    action_state: Single<&ActionState<GameAction>, With<Player>>,
) {
    let braking = if action_state.pressed(&GameAction::Brake) {
        3.0
    } else {
        1.0
    };
    let decel = BASE_DECELERATION / player_stats.speed as f32 * braking;
    **scroll_speed = (**scroll_speed - decel * time.delta_secs()).max(0.0);

    if **scroll_speed == 0.0 {
        next_state.set(LugeState::Loadout);
    }
}

fn reset_scroll_speed(mut scroll_speed: ResMut<ScrollSpeed>) {
    *scroll_speed = ScrollSpeed::default();
}

fn reset_luge(
    resolution: Res<Resolution>,
    mut player_lane: ResMut<PlayerLane>,
    mut luigee: Single<&mut Transform, With<LuigeeSprite>>,
    mut lanes: Query<&mut Transform, (With<LaneSprite>, Without<LuigeeSprite>)>,
) {
    **player_lane = LaneLocation::default();
    luigee.translation.x = 0.0;

    let offsets = [0.0, 360.0 * resolution.scale()];
    for (i, mut transform) in lanes.iter_mut().enumerate() {
        transform.translation.y = offsets[i];
    }
}
