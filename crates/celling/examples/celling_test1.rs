use std::{sync::mpsc, thread};

use bevy::{
    prelude::*,
    window::WindowResolution, core::FrameCount, render::{camera::ScalingMode, render_resource::FragmentState},
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
};
use bevy_rapier2d::prelude::*;
use rand::{Rng, seq::SliceRandom};
use bevy_framepace;
use bevy_inspector_egui::{quick::WorldInspectorPlugin, egui::epaint::image};

use celling::prelude::*;
use celling::prelude::load::SpawnImageSpriteEvent;
use celling::prelude::rigids::RigidizeEvent;

const APP_NAME: &str = "moob";
const WINDOW_W: f32 = 1920.0;
const WINDOW_H: f32 = 1080.0;
const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);
const NORMAL_BUTTON: Color = Color::rgb(0.19, 0.28, 0.31);

#[derive(Resource)]
struct MousePress(CellType);

#[derive(Component)]
struct Player;

fn main() {
    App::new()
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(MousePress(CellType::Sand))
        .add_event::<SpawnImageSpriteEvent>()
        .add_event::<RigidizeEvent>()
        .add_plugins(CellingPlugin::default())
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: APP_NAME.to_string(),
                resolution: WindowResolution::new(WINDOW_W, WINDOW_H).with_scale_factor_override(1.0),
                ..default()
            }),
            ..default()
        }
    ))
    .add_plugins(bevy_framepace::FramepacePlugin)
    // .add_plugins((LogDiagnosticsPlugin::default(),FrameTimeDiagnosticsPlugin::default()))
    .add_systems(Startup, (setup, handle))
    .add_systems(Update, (handle_click, button_system, bevy::window::close_on_esc, player_system))
    
    .add_plugins(WorldInspectorPlugin::default())
    .add_systems(Update, camera::movement)
    .run();
}

fn setup(
    mut commands: Commands,
    mut settings: ResMut<bevy_framepace::FramepaceSettings>
    ) {
    // Camera
    let mut camera = Camera2dBundle::default();
    camera.projection.scaling_mode = ScalingMode::Fixed { width: 1920., height: 1080. };
    commands.spawn(camera);

    use bevy_framepace::Limiter;
    settings.limiter = Limiter::from_framerate(30.0);
}

const SAND_COLOR: Color = Color::rgb(0.0, 0.0, 0.0);
const LIQUIP_COLOR: Color = Color::rgb(0.33, 0.42, 0.71);
const GAS_COLOR: Color = Color::rgb(220.0/255.0, 221.0/255.0, 213.0/255.0);

fn spwan_wall(cmds: &mut Commands, map: &mut CellsMap, from_x: i32, to_x: i32, from_y: i32, to_y: i32) {
    for i in from_x..=to_x {
        for j in from_y..=to_y {
            let cell_bundle = CellBundle {
                c: Cell::Stable,
                d: Density(1),
                cd: CellDir::None,
            };
            if let Some(eid) = create_cell(cmds, map, cell_bundle, i * PIXEL_SIZE, j * PIXEL_SIZE, SAND_COLOR) {
                cmds.entity(eid).insert(Silent);
            }
        }
    }

    let pixelize_from_x = from_x * PIXEL_SIZE;
    let pixelize_to_x = to_x * PIXEL_SIZE;
    let pixelize_from_y = from_y * PIXEL_SIZE;
    let pixelize_to_y = to_y * PIXEL_SIZE;
    if let Some(t) = match (to_x - from_x, to_y - from_y) {
        (0, n) if n != 0 => {
            let x = get_fix_pos(pixelize_to_x) as f32;
            let y = get_fix_pos((get_fix_pos(pixelize_to_y) + get_fix_pos(pixelize_from_y)) / 2) as f32;
            Some(Transform::from_xyz(x, y, 0.0))
        }
        (n, 0) if n != 0 => {
            let x = get_fix_pos((get_fix_pos(pixelize_to_x) + get_fix_pos(pixelize_from_x)) / 2) as f32;
            let y = get_fix_pos(pixelize_to_y) as f32;
            Some(Transform::from_xyz(x, y, 0.0))
        }
        (0, 0) => {
            let x = get_fix_pos((get_fix_pos(pixelize_to_x) + get_fix_pos(pixelize_from_x)) / 2) as f32;
            let y = get_fix_pos((get_fix_pos(pixelize_to_y) + get_fix_pos(pixelize_from_y)) / 2) as f32;
            Some(Transform::from_xyz(x, y, 0.0))
        }
        _ => {None}
    } {
        let half_x = (((to_x - from_x + 1)  * PIXEL_SIZE) / 2) as f32;
        let half_y = (((to_y - from_y + 1)  * PIXEL_SIZE) / 2) as f32;
        cmds
            .spawn(Collider::cuboid(half_x, half_y))
            .insert(TransformBundle::from(t));
    }
}

pub fn handle(
    mut commands: Commands,
    mut map: ResMut<CellsMap>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    images: Res<Assets<Image>>,
    mut spawn_events: EventWriter<SpawnImageSpriteEvent>,
)
{
    spwan_wall(&mut commands, &mut map, -1000, 1000, -100, -100);
    // spwan_wall(&mut commands, &mut map, -100, 100, -50, -50);
    // spwan_wall(&mut commands, &mut map, -100, 100, -30, -30);
    // spwan_wall(&mut commands, &mut map, -100, 100, 15, 15);
    // spwan_wall(&mut commands, &mut map, -100, -100, 0, 30);
    // spwan_wall(&mut commands, &mut map, -50, -50, 0, 30);

    commands
    .spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Start,
            justify_content: JustifyContent::Default,
            ..default()
        },
        ..default()
    })
    .with_children(|parent| {
        parent
            .spawn((ButtonBundle {
                style: Style {
                    width: Val::Px(150.0),
                    height: Val::Px(65.0),
                    border: UiRect::all(Val::Px(5.0)),
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    ..default()
                },
                border_color: BorderColor(Color::BLACK),
                background_color: NORMAL_BUTTON.into(),
                ..default()
            }, CellType::Sand))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "SAND",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                ));
            });
    }).with_children(|parent| {
        parent
            .spawn((ButtonBundle {
                style: Style {
                    width: Val::Px(150.0),
                    height: Val::Px(65.0),
                    border: UiRect::all(Val::Px(5.0)),
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    ..default()
                },
                border_color: BorderColor(Color::BLACK),
                background_color: NORMAL_BUTTON.into(),
                ..default()
            }, CellType::Liquip))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "LIQUIP",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                ));
            });
    }).with_children(|parent| {
        parent
            .spawn((ButtonBundle {
                style: Style {
                    width: Val::Px(150.0),
                    height: Val::Px(65.0),
                    border: UiRect::all(Val::Px(5.0)),
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    ..default()
                },
                border_color: BorderColor(Color::BLACK),
                background_color: NORMAL_BUTTON.into(),
                ..default()
            }, CellType::Gas))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "GAS",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                ));
            });
    });

    /* Create the bouncing ball. */
    commands
        .spawn((
            RigidBody::Dynamic,
            Collider::ball(50.0),
            TransformBundle::from(Transform::from_xyz(-600.0, 1000.0, 0.0)),
            GravityScale(1.0),
            ExternalForce {
                force: Vec2::new(0.0, -1000.0),
                // torque: 140.0,
                ..default()
            },
            Velocity::default(),
            RigidCheckField::new(50, 50),
            Player
        ));

    spawn_events.send(SpawnImageSpriteEvent::new(Po::create(0, 100), "tree_sprite_2.png".to_string()))
}

fn handle_click(
    mut commands: Commands,
    input: Res<Input<MouseButton>>,
    windows: Query<&Window>,
    mouse_press: ResMut<MousePress>,
    mut map: ResMut<CellsMap>,
) {
    let cd = CellDir::new();
    if input.pressed(MouseButton::Left) {
        if let Some(cursor_position) = windows.single().cursor_position() {
            let ct: CellType = mouse_press.0;
            let (c, color, d, cd) = match ct {
                CellType::Sand => {
                    (Cell::Sand, SAND_COLOR, Density(1), CellDir::None)
                }
                CellType::Liquip => {
                    (Cell::Liquip, LIQUIP_COLOR, Density(1), cd)
                }
                CellType::Gas => {
                    (Cell::Gas, GAS_COLOR, Density(1), cd)
                }
                _ => {
                    (Cell::Stable, GAS_COLOR, Density(1), cd)
                }
            };
            let cell_bundle = CellBundle {
                c: c,
                d: d,
                cd: cd,
            };
            let x = (cursor_position.x - WINDOW_W / 2.0) as i32;
            let y = (WINDOW_H / 2.0 - cursor_position.y) as i32;
            let tmp = get_cell_create_pos(x, y);
            let p = Po {x: tmp.0, y: tmp.1};
            if map.get(&p) == None {
                create_cell(&mut commands, &mut map, cell_bundle, x, y, color);
            }
        }
    }
}

fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &Children,
            &CellType,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut mouse_press: ResMut<MousePress>
) {
    for (interaction, mut color, children, cell_type) in &mut interaction_query {
        let mut _text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                mouse_press.0 = *cell_type;
                *color = PRESSED_BUTTON.into();
            }
            _ => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

const PLANE_MOVE_TRANSLATION: f32 = 6.18;
fn player_system(
    mut _cmds: Commands,
    mut query: Query<&mut Transform, With<Player>>,
    input: Res<Input<KeyCode>>,
) {
    
    let mut player_transform = query.get_single_mut().unwrap();
    for key in input.get_pressed() {
        match *key {
            KeyCode::A => {
                player_transform.translation.x -= PLANE_MOVE_TRANSLATION;
            }
            KeyCode::D => {
                player_transform.translation.x += PLANE_MOVE_TRANSLATION;
            }
            KeyCode::W => {
                player_transform.translation.y += PLANE_MOVE_TRANSLATION;
            }
            KeyCode::S => {
                player_transform.translation.y -= PLANE_MOVE_TRANSLATION;
            }
            _ => {}
        }
    }
}