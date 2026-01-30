use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn name(&self) -> &str {
        "Player Plugin"
    }

    fn build(&self, app: &mut App) {
        app;
    }
}

#[derive(Component)]
pub struct Player;
