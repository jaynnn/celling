use bevy::{
    prelude::*,
    window::WindowResolution
};

mod utils;
pub use utils::*;


const APP_NAME: &str = "moob";
const WINDOW_W: f32 = 1920.0;
const WINDOW_H: f32 = 1080.0;
const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

#[derive(Event, Default)]
struct CollisionEvent;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Component)]
struct Ball;

fn main() {
    App::new()
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: APP_NAME.to_string(),
            resolution: WindowResolution::new(WINDOW_W, WINDOW_H).with_scale_factor_override(1.0),
            ..default()
        }),
        ..default()
    }))
    .add_systems(Startup, setup)
    .run();
}

fn setup(
    mut commands: Commands,
    ) {
    // Camera
    commands.spawn(Camera2dBundle::default());
}