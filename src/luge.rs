use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;

use crate::{
    GameState, LugeState, Resolution,
    actions::LugeAction,
    loading::SpriteAssets,
    player::{Player, PlayerStats},
    ui::UiColor,
};

pub struct LugePlugin;

impl Plugin for LugePlugin {
    fn name(&self) -> &str {
        "Luge Plugin"
    }

    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Playing),
            (spawn_luigee, spawn_lanes, spawn_slick_ui, update_lanes),
        )
        .add_systems(
            Update,
            (move_luigee, update_luigee_sprite, scroll_lanes).run_if(in_state(LugeState::Launched)),
        )
        .insert_resource(Lanes::default())
        .insert_resource(PlayerLane::default())
        .insert_resource(ScrollSpeed::default());
    }
}

#[derive(Component)]
struct LuigeeSprite;

#[derive(Component)]
struct LaneSprite;

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
    action_state: Single<&ActionState<LugeAction>, With<Player>>,
) {
    if action_state.just_pressed(&LugeAction::Left) {
        info!("Luge Action Left");
        **player_lane = player_lane.shift_left();
    }

    if action_state.just_pressed(&LugeAction::Right) {
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

fn spawn_slick_ui(mut commands: Commands, resolution: Res<Resolution>, sprites: Res<SpriteAssets>) {
    let s = resolution.ui_scale();
    let border = 8.0 * s;

    commands
        .spawn((
            Name::new("Slick UI Parent"),
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
            parent.spawn((
                Name::new("Rick Text Container"),
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(65.0),
                    border: UiRect::all(Val::Px(border)),
                    padding: UiRect::axes(Val::Px(0.0), Val::Px(0.0)),
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(0.0),
                    ..default()
                },
                BorderColor::all(UiColor::Light.color()),
            ));
        });
}
