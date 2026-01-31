// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;
use slick_ricks_luge_lounge::GamePlugin;

fn main() -> AppExit {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Slick Rick's Luge Lounge".to_string(),
                        canvas: Some("#bevy".to_owned()),
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin {
                    meta_check: bevy::asset::AssetMetaCheck::Never,
                    ..default()
                })
                // pixel perfect rendering
                .set(ImagePlugin::default_nearest()),
            GamePlugin,
        ))
        .run()
}
