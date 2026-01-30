// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::{App, AppExit};
use slick_ricks_luge_lounge::GamePlugin;

fn main() -> AppExit {
    App::new().add_plugins(GamePlugin).run()
}
