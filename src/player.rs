use bevy::prelude::*;
use leafwing_input_manager::prelude::InputMap;

use crate::{GameState, actions::LugeAction};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn name(&self) -> &str {
        "Player Plugin"
    }

    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_player)
            .insert_resource(PlayerStats::default());
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Resource)]
pub struct PlayerStats {
    pub attack: i32,
    pub defence: i32,
    pub speed: i32,
    pub luck: i32,
}

impl Default for PlayerStats {
    fn default() -> Self {
        Self {
            attack: 1,
            defence: 1,
            speed: 1,
            luck: 1,
        }
    }
}

impl Player {
    fn default_input_map() -> InputMap<LugeAction> {
        use LugeAction::*;

        let mut input_map = InputMap::default();

        // Movement
        input_map.insert(Left, KeyCode::ArrowLeft);
        input_map.insert(Right, KeyCode::ArrowRight);

        input_map
    }
}

fn spawn_player(mut commands: Commands) {
    commands.spawn((Player, Player::default_input_map()));
    info!("Spawned Player.");
}
