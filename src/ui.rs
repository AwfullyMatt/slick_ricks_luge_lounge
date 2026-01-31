use bevy::prelude::*;

use crate::GameState;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            button_click_handler
                .run_if(in_state(GameState::Menu).or(in_state(GameState::Settings))),
        );
    }
}

#[allow(dead_code)]
pub enum UiColor {
    Darkest,
    Darker,
    Dark,
    Light,
    Lighter,
    Lightest,
}

impl UiColor {
    pub fn color(&self) -> Color {
        use UiColor::*;
        match self {
            Darkest => Color::srgb(0.192, 0.212, 0.220),
            Darker => Color::srgb(0.196, 0.325, 0.373),
            Dark => Color::srgb(0.039, 0.467, 0.478),
            Light => Color::srgb(0.290, 0.659, 0.506),
            Lighter => Color::srgb(0.451, 0.937, 0.910),
            Lightest => Color::srgb(0.925, 0.953, 0.690),
        }
    }
}

#[derive(Component, Clone)]
pub struct ButtonColors {
    pub normal: Color,
    pub hovered: Color,
}

impl Default for ButtonColors {
    fn default() -> Self {
        ButtonColors {
            normal: UiColor::Light.color(),
            hovered: UiColor::Lighter.color(),
        }
    }
}

#[derive(Component)]
pub struct ChangeState(pub GameState);

#[derive(Component)]
pub struct OpenLink(pub &'static str);

// helper method to make fonts fit
pub fn font_size_for(width: f32, height: f32, text: &str) -> f32 {
    let from_height = height * 0.7;
    let from_width = width / (text.len() as f32 * 0.65);
    from_height.min(from_width)
}

pub fn button_click_handler(
    mut next_state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &ButtonColors,
            Option<&ChangeState>,
            Option<&OpenLink>,
        ),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, button_colors, change_state, open_link) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if let Some(state) = change_state {
                    next_state.set(state.0.clone());
                } else if let Some(link) = open_link
                    && let Err(error) = webbrowser::open(link.0)
                {
                    warn!("Failed to open link {error:?}");
                }
            }
            Interaction::Hovered => {
                *color = button_colors.hovered.into();
            }
            Interaction::None => {
                *color = button_colors.normal.into();
            }
        }
    }
}
