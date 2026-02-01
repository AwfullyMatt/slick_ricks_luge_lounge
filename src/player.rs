use bevy::prelude::*;
use leafwing_input_manager::prelude::InputMap;

use crate::actions::GameAction;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn name(&self) -> &str {
        "Player Plugin"
    }

    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerStats::default());
    }
}

#[derive(Component)]
pub struct Player;

#[allow(dead_code)]
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
    pub fn default_input_map() -> InputMap<GameAction> {
        use GameAction::*;

        let mut input_map = InputMap::default();

        // Movement
        input_map.insert(Left, KeyCode::ArrowLeft);
        input_map.insert(Right, KeyCode::ArrowRight);

        // Dialogue
        input_map.insert(Continue, KeyCode::Space);
        input_map.insert(Continue, MouseButton::Left);

        input_map
    }
}
