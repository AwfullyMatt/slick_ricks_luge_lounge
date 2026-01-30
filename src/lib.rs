#![allow(clippy::type_complexity)]

mod audio;
mod loading;
mod luge;
mod menu;
mod player;
mod settings;
mod ui;

use crate::audio::InternalAudioPlugin;
use crate::loading::LoadingPlugin;
use crate::luge::LugePlugin;
use crate::menu::MenuPlugin;
use crate::player::PlayerPlugin;
use crate::settings::SettingsPlugin;
use crate::ui::UiColor;

use bevy::app::App;
use bevy::prelude::*;

//#[cfg(debug_assertions)]
//use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    #[default]
    Loading,
    Playing,
    Menu,
    Settings,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        // Insert Clearcolor backaground
        app.insert_resource(ClearColor(UiColor::Lightest.linear_rgb()));
        // Confirgure Default plugins
        app.add_plugins(
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
                .set(ImagePlugin::default_nearest()),
        );
        // Configure custom plugins
        app.add_plugins((
            LoadingPlugin,
            MenuPlugin,
            InternalAudioPlugin,
            PlayerPlugin,
            LugePlugin,
            SettingsPlugin,
        ));
        // Initialize gamestates
        app.init_state::<GameState>();

        // #[cfg(debug_assertions)]
        // {
        //     app.add_plugins((
        //         FrameTimeDiagnosticsPlugin::default(),
        //         LogDiagnosticsPlugin::default(),
        //     ));
        // }
    }
}
