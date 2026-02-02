use bevy::prelude::*;
use rand::Rng;

use crate::{Resolution, loading::SpriteAssets, player::PlayerStats};

use super::{LaneLocation, Lanes, LuigeeSprite, PlayerLane, ScrollSpeed};

#[derive(Component)]
pub(super) struct LaneOccupant {
    pub lane: LaneLocation,
}

#[derive(Component)]
pub(super) struct Coin {
    pub value: u32,
}

#[derive(Resource, Default, Deref, DerefMut)]
pub(super) struct PlayerCoins(pub u32);

#[derive(Resource)]
pub(super) struct CoinAtlasLayout(Handle<TextureAtlasLayout>);

#[derive(Resource, Deref, DerefMut)]
pub(super) struct SpawnTimer(Timer);

pub(super) fn init_coin_atlas(
    mut commands: Commands,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 3, 1, None, None);
    let handle = layouts.add(layout);
    commands.insert_resource(CoinAtlasLayout(handle));
}

pub(super) fn init_spawn_timer(mut commands: Commands, player_stats: Res<PlayerStats>) {
    let luck = player_stats.luck as f32;
    let interval = (2.5 / (1.0 + luck * 0.1)).max(0.8);
    commands.insert_resource(SpawnTimer(Timer::from_seconds(
        interval,
        TimerMode::Repeating,
    )));
}

pub(super) fn spawn_initial_coin(
    mut commands: Commands,
    resolution: Res<Resolution>,
    sprites: Res<SpriteAssets>,
    lanes: Res<Lanes>,
    coin_atlas: Option<Res<CoinAtlasLayout>>,
) {
    let Some(coin_atlas) = coin_atlas else {
        return;
    };

    let y = resolution.vec2().y / 2.0 + 50.0;
    let lane = LaneLocation::Center;
    let lane_x = lanes.x_for(lane);

    commands.spawn((
        Sprite::from_atlas_image(
            sprites.coins.clone(),
            TextureAtlas {
                layout: coin_atlas.0.clone(),
                index: 0,
            },
        ),
        Transform {
            translation: Vec3::new(lane_x, y, 0.5),
            scale: Vec3::splat(resolution.scale()),
            ..default()
        },
        LaneOccupant { lane },
        Coin { value: 1 },
    ));
}

#[allow(clippy::too_many_arguments)]
pub(super) fn spawn_coins(
    mut commands: Commands,
    time: Res<Time>,
    resolution: Res<Resolution>,
    sprites: Res<SpriteAssets>,
    player_stats: Res<PlayerStats>,
    lanes: Res<Lanes>,
    coin_atlas: Option<Res<CoinAtlasLayout>>,
    mut spawn_timer: Option<ResMut<SpawnTimer>>,
) {
    let (Some(coin_atlas), Some(ref mut spawn_timer)) = (coin_atlas, spawn_timer.as_mut()) else {
        return;
    };

    spawn_timer.tick(time.delta());
    if !spawn_timer.just_finished() {
        return;
    }

    let mut rng = rand::rng();
    let luck = player_stats.luck as f32;

    // Pick random lane
    let lane = match rng.random_range(0..3) {
        0 => LaneLocation::Left,
        1 => LaneLocation::Center,
        _ => LaneLocation::Right,
    };

    // Weighted coin type selection
    let weight_1 = 100.0_f32;
    let weight_5 = 10.0 + luck * 5.0;
    let weight_25 = 2.0 + luck * 2.0;
    let total = weight_1 + weight_5 + weight_25;

    let roll: f32 = rng.random_range(0.0..total);
    let (atlas_index, value) = if roll < weight_1 {
        (0, 1)
    } else if roll < weight_1 + weight_5 {
        (1, 5)
    } else {
        (2, 25)
    };

    let lane_x = lanes.x_for(lane);
    let y = resolution.vec2().y / 2.0 + 50.0;

    commands.spawn((
        Sprite::from_atlas_image(
            sprites.coins.clone(),
            TextureAtlas {
                layout: coin_atlas.0.clone(),
                index: atlas_index,
            },
        ),
        Transform {
            translation: Vec3::new(lane_x, y, 0.5),
            scale: Vec3::splat(resolution.scale()),
            ..default()
        },
        LaneOccupant { lane },
        Coin { value },
    ));
}

pub(super) fn scroll_occupants(
    time: Res<Time>,
    player_stats: Res<PlayerStats>,
    scroll_speed: Res<ScrollSpeed>,
    mut query: Query<&mut Transform, With<LaneOccupant>>,
) {
    let speed = **scroll_speed * (player_stats.speed as f32 / 10.0);
    let delta = speed * time.delta_secs();
    for mut transform in query.iter_mut() {
        transform.translation.y -= delta;
    }
}

pub(super) fn collect_coins(
    mut commands: Commands,
    resolution: Res<Resolution>,
    player_lane: Res<PlayerLane>,
    mut player_coins: ResMut<PlayerCoins>,
    coins: Query<(Entity, &LaneOccupant, &Coin, &Transform)>,
    luigee: Single<&Transform, (With<LuigeeSprite>, Without<LaneOccupant>)>,
) {
    let threshold = 40.0 * resolution.scale();
    let luigee_y = luigee.translation.y;

    for (entity, occupant, coin, transform) in coins.iter() {
        if occupant.lane == **player_lane && (transform.translation.y - luigee_y).abs() < threshold
        {
            **player_coins += coin.value;
            commands.entity(entity).despawn();
        }
    }
}

pub(super) fn despawn_offscreen(
    mut commands: Commands,
    resolution: Res<Resolution>,
    query: Query<(Entity, &Transform), With<LaneOccupant>>,
) {
    let cutoff = -(resolution.vec2().y / 2.0 + 100.0);
    for (entity, transform) in query.iter() {
        if transform.translation.y < cutoff {
            commands.entity(entity).despawn();
        }
    }
}

pub(super) fn cleanup_spawner(mut commands: Commands, query: Query<Entity, With<LaneOccupant>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
    commands.remove_resource::<SpawnTimer>();
}

pub(super) fn reset_player_coins(mut player_coins: ResMut<PlayerCoins>) {
    **player_coins = 0;
}
