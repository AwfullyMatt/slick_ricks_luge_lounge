use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct ActionsPlugin;

impl Plugin for ActionsPlugin {
    fn name(&self) -> &str {
        "Actions Plugin"
    }

    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<GameAction>::default());
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum GameAction {
    // Movement
    Left,
    Right,
    Brake,
    // Ui
    Continue,
}
