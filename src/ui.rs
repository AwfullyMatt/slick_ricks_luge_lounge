use bevy::prelude::*;

pub enum UiColor {
    Darkest,
    Darker,
    Dark,
    Light,
    Lighter,
    Lightest,
}

impl UiColor {
    pub fn linear_rgb(&self) -> Color {
        use UiColor::*;
        match self {
            Darkest => Color::srgb(0.192, 0.212, 0.220),
            Darker => Color::srgb(0.196, 0.325, 0.373),
            Dark => Color::srgb(0.039, 0.467, 0.478),
            Light => Color::srgb(0.290, 0.659, 0.506),
            Lighter => Color::srgb(0.451, 0.937, 0.910),
            Lightest => Color::srgb(0.925, 0.953, 0.690),
            _ => Color::srgb(0.0, 0.0, 0.0),
        }
    }
}
