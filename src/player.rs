use bevy::prelude::*;
use leafwing_input_manager::prelude::InputMap;

use crate::{GameState, actions::LugeAction};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn name(&self) -> &str {
        "Player Plugin"
    }

    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_player);
    }
}

#[derive(Component)]
pub struct Player;

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
