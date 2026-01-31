use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
//use bevy_kira_audio::AudioSource;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::Menu)
                .load_collection::<AudioAssets>()
                .load_collection::<TextureAssets>()
                .load_collection::<FontAssets>()
                .load_collection::<SpriteAssets>(),
        );
    }
}

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {}

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(path = "textures/bevy.png")]
    pub bevy: Handle<Image>,

    #[asset(path = "textures/github.png")]
    pub github: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct SpriteAssets {
    #[asset(path = "sprites/luigee.png")]
    pub luigee: Handle<Image>,

    #[allow(dead_code)]
    #[asset(path = "sprites/slick_rick.png")]
    pub slick_rick: Handle<Image>,

    #[asset(path = "sprites/title.png")]
    pub title: Handle<Image>,

    #[asset(path = "sprites/lanes.png")]
    pub lanes: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct FontAssets {
    #[asset(path = "fonts/Tiny5-Regular.ttf")]
    pub tiny5: Handle<Font>,
}
