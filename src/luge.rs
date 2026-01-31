use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;

use crate::{
    GameState, actions::LugeAction, loading::SpriteAssets, player::Player, settings::Settings,
};

pub struct LugePlugin;

impl Plugin for LugePlugin {
    fn name(&self) -> &str {
        "Luge Plugin"
    }

    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Playing),
            (spawn_luigee, spawn_lanes_sprite, update_lanes),
        )
        .add_systems(
            Update,
            (move_luigee, update_luigee_sprite).run_if(in_state(GameState::Playing)),
        )
        .add_systems(OnExit(GameState::Playing), cleanup_luge)
        .insert_resource(Lanes::default())
        .insert_resource(PlayerLane::default());
    }
}

#[derive(Component)]
struct LugeCleanup;

#[derive(Component)]
struct LuigeeSprite;

fn spawn_luigee(mut commands: Commands, settings: Res<Settings>, sprites: Res<SpriteAssets>) {
    let y = -(settings.resolution.vec2().y / 3.0);
    commands.spawn((
        Player,
        Sprite::from_image(sprites.luigee.clone()),
        Transform {
            translation: Vec3::new(0.0, y, 0.0),
            scale: Vec3::splat(3.0),
            ..default()
        },
        LuigeeSprite,
    ));
}

fn spawn_lanes_sprite(mut commands: Commands, sprites: Res<SpriteAssets>, settings: Res<Settings>) {
    commands.spawn((
        LugeCleanup,
        Sprite::from_image(sprites.lanes.clone()),
        Transform {
            translation: Vec3::new(0.0, 0.0, -1.0),
            scale: Vec3::splat(settings.resolution.scale()),
            ..default()
        },
    ));
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
    fn init(settings: Res<Settings>) -> Self {
        let transform = settings.resolution.calculate_lanes();
        Self {
            left: Lane {
                location: LaneLocation::Left,
                x: transform.0,
            },
            center: Lane {
                location: LaneLocation::Center,
                x: 0.0,
            },
            right: Lane {
                location: LaneLocation::Right,
                x: transform.1,
            },
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
    pub location: LaneLocation,
    pub x: f32,
}

#[derive(Default, Copy, Clone)]
pub enum LaneLocation {
    Left,
    #[default]
    Center,
    Right,
}

impl LaneLocation {
    fn _id(&self) -> usize {
        use LaneLocation::*;
        match self {
            Left => 0,
            Center => 1,
            Right => 2,
        }
    }

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

fn update_lanes(mut lanes: ResMut<Lanes>, settings: Res<Settings>) {
    *lanes = Lanes::init(settings);
}

fn cleanup_luge(mut commands: Commands, query_cleanup: Query<Entity, With<LugeCleanup>>) {
    for entity in query_cleanup.iter() {
        commands.entity(entity).despawn();
    }
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
    };
}
