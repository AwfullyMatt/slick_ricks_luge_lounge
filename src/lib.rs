#![allow(clippy::type_complexity)]

mod actions;
mod audio;
mod loading;
mod luge;
mod menu;
mod player;
mod settings;
mod ui;

use crate::actions::ActionsPlugin;
use crate::audio::InternalAudioPlugin;
use crate::loading::LoadingPlugin;
use crate::luge::LugePlugin;
use crate::menu::MenuPlugin;
use crate::player::PlayerPlugin;
use crate::settings::SettingsPlugin;
use crate::ui::{UiColor, UiPlugin};

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

#[derive(Resource, Default, Clone, Copy)]
pub enum Resolution {
    Sd,
    #[default]
    Hd,
    Qhd,
    Uhd,
}

impl Resolution {
    pub const RESOLUTIONS: [Self; 4] = [Self::Sd, Self::Hd, Self::Qhd, Self::Uhd];

    pub fn vec2(&self) -> Vec2 {
        use Resolution::*;
        match self {
            Sd => Vec2::new(1280.0, 720.0),
            Hd => Vec2::new(1920.0, 1080.0),
            Qhd => Vec2::new(2560.0, 1440.0),
            Uhd => Vec2::new(3840.0, 2160.0),
        }
    }

    pub fn label(&self) -> &'static str {
        use Resolution::*;
        match self {
            Sd => "720p",
            Hd => "1080p",
            Qhd => "1440p",
            Uhd => "4k",
        }
    }

    pub fn calculate_lanes(&self) -> (f32, f32) {
        let offset = self.scale() * 61.0;
        (-offset, offset)
    }

    pub fn scale(&self) -> f32 {
        match self {
            Resolution::Sd => 2.0,
            Resolution::Hd => 3.0,
            Resolution::Qhd => 4.0,
            Resolution::Uhd => 6.0,
        }
    }

    pub fn ui_scale(&self) -> f32 {
        self.scale() / 3.0
    }
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        // Insert ClearColor background
        app.insert_resource(ClearColor(UiColor::Lightest.color()));
        // Configure custom plugins
        app.add_plugins((
            LoadingPlugin,
            MenuPlugin,
            UiPlugin,
            InternalAudioPlugin,
            PlayerPlugin,
            ActionsPlugin,
            LugePlugin,
            SettingsPlugin,
        ));
        // Initialize gamestates
        app.init_state::<GameState>();
        // Spawn camera
        app.add_systems(Startup, spawn_camera);

        // #[cfg(debug_assertions)]
        // {
        //     app.add_plugins((
        //         FrameTimeDiagnosticsPlugin::default(),
        //         LogDiagnosticsPlugin::default(),
        //     ));
        // }
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2d, Msaa::Off));
}
