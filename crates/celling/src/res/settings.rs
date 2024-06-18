use bevy::prelude::*;

#[derive(Resource)]
pub struct Settings {
    marching_squares_simplify_eps: f64,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            marching_squares_simplify_eps: 1e-9,
        }
    }
}