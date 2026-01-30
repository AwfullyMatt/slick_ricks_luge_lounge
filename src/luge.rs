use bevy::prelude::*;

use crate::{
    GameState,
    loading::SpriteAssets,
    player::Player,
    settings::{Resolution, Settings},
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
        .add_systems(OnExit(GameState::Playing), cleanup_luge)
        .insert_resource(Lanes::default());
    }
}

#[derive(Component)]
struct LugeCleanup;

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
                id: 0,
                location: LaneLocation::Left,
                x: transform.0,
            },
            center: Lane {
                id: 1,
                location: LaneLocation::Center,
                x: 0.0,
            },
            right: Lane {
                id: 2,
                location: LaneLocation::Right,
                x: transform.1,
            },
        }
    }
}

#[derive(Default)]
pub struct Lane {
    pub id: usize,
    pub location: LaneLocation,
    pub x: f32,
}

#[derive(Default)]
pub enum LaneLocation {
    #[default]
    Left,
    Center,
    Right,
}

fn update_lanes(mut lanes: ResMut<Lanes>, settings: Res<Settings>) {
    *lanes = Lanes::init(settings);
}

fn cleanup_luge(mut commands: Commands, query_cleanup: Query<Entity, With<LugeCleanup>>) {
    for entity in query_cleanup.iter() {
        commands.entity(entity).despawn();
    }
}
