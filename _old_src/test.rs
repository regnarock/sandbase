use bevy::{
    input::mouse::{MouseButton},
    prelude::*,
    sprite::MaterialMesh2dBundle,
};
use std::collections::HashMap;

const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 600.0;
const PIXEL_SIZE: usize = 10;
const GAME_WORLD_WIDTH: usize = (SCREEN_WIDTH / PIXEL_SIZE as f32) as usize;
const GAME_WORLD_HEIGHT: usize = (SCREEN_HEIGHT / PIXEL_SIZE as f32) as usize;
const PIXELS: usize = GAME_WORLD_WIDTH * GAME_WORLD_HEIGHT;
const SAND: Color = Color::rgb(0.761, 0.698, 0.);
const WATER: Color = Color::rgb(0., 0.749, 1.);

#[derive(Component, Copy, Clone, Debug)]
struct Voxel {
    size: usize,
    speed: f32,
}

struct GameWorld {
    voxels: [Option<Voxel>; PIXELS],
}

impl Resource for GameWorld {}

impl Default for GameWorld {
    fn default() -> Self {
        Self {
            voxels: [None; GAME_WORLD_WIDTH * GAME_WORLD_HEIGHT]
        }
    }
}

#[derive(Component)]
struct Sand();

#[derive(Component)]
struct Water();

#[derive(Default)]
struct MouseLocScreen(Vec2);

#[derive(Default)]
struct MouseLocWorld(Vec2);

impl Resource for MouseLocScreen {}

impl Resource for MouseLocWorld {}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Welcome to Sandbase!".into(),
                resolution: (SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32).into(),
                ..default()
            }),
            ..default()
        }))
        .init_resource::<MouseLocScreen>()
        .init_resource::<MouseLocWorld>()
        .init_resource::<GameWorld>()
        .add_startup_system(setup)
        .add_startup_system(setup_ui)
        .add_system(mouse_movement_updating_system)
        .add_system(handle_input)
        .add_system(update_voxel_positions_system)
        .run();
}

fn setup(
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_translation(Vec3::new(
            SCREEN_WIDTH / 2.0,
            SCREEN_HEIGHT / 2.0,
            0.0,
        )),
        ..Default::default()
    });
}

// Spawns the UI
fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Node that fills entire background
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::width(Val::Percent(100.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                        position: UiRect {
                            left: Val::Px(-SCREEN_WIDTH / 2.0),
                            top: Val::Px(-SCREEN_HEIGHT / 2.0),
                            ..default()
                        },
                        margin: UiRect {
                            left: Val::Px(150.),
                            top: Val::Px(65.),
                            ..default()
                        },
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: SAND.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Sand",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        });
}

fn update_voxel_positions_system(
    mut commands: Commands,
    mut world: ResMut<GameWorld>,
    mut query: Query<(Entity, &mut Transform, &mut Voxel)>,
) {
    for (entity, mut transform, mut voxel) in query.iter_mut() {
        let current_index = get_index_from_pos(transform.translation.truncate());

        if current_index > GAME_WORLD_WIDTH {
            let mut new_index = current_index - GAME_WORLD_WIDTH;
            let mut moved = false;
            let old_voxel = world.voxels[current_index];
            let new_voxel = world.voxels[new_index];
            if new_voxel.is_none() {
                moved = true;
            } else if is_left_empty(&world, current_index) {
                new_index -= 1;
                moved = true;
            } else if is_right_empty(&world, current_index) {
                new_index += 1;
                moved = true;
            }

            if moved {
                world.voxels[current_index] = None;
                world.voxels[new_index] = old_voxel;
                let new_pos = get_pos_from_index(new_index);
                transform.translation = Vec3::new(new_pos.x, new_pos.y, 0.0);
            }
        }
    }
}

fn is_left_empty(world: &GameWorld, index: usize) -> bool {
    let left_index = index - 1 - GAME_WORLD_WIDTH;
    left_index >= 0 && world.voxels[left_index].is_none()
}

fn is_right_empty(world: &GameWorld, index: usize) -> bool {
    let right_index = index + 1 - GAME_WORLD_WIDTH;
    right_index < PIXELS && world.voxels[right_index].is_none()
}

fn get_index_from_pos(pos: Vec2) -> usize {
    let x = pos.x as usize / PIXEL_SIZE;
    let y = pos.y as usize / PIXEL_SIZE;
    y * GAME_WORLD_WIDTH + x
}

fn get_pos_from_index(index: usize) -> Vec2 {
    let x = index % GAME_WORLD_WIDTH;
    let y = index / GAME_WORLD_WIDTH;
    Vec2::new((x * PIXEL_SIZE) as f32, (y * PIXEL_SIZE) as f32)
}

fn handle_input(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>, // Add this line
    mouse_loc: Res<MouseLocWorld>,
    mut world: ResMut<GameWorld>,
    mouse_button_input: Res<Input<MouseButton>>,
    mut buttons: Query<(&mut BackgroundColor, &Button, &Children)>,
    mut texts: Query<&mut Text>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) || mouse_button_input.pressed(MouseButton::Left) {
        let pos = mouse_loc.0;
        let index = get_index_from_pos(pos);
        let snaped_pos = get_pos_from_index(index);
        if world.voxels[index].is_none() {
            let voxel = Voxel { size: PIXEL_SIZE, speed: PIXEL_SIZE as f32 };
            world.voxels[index] = Some(voxel);
            let pos = get_pos_from_index(index);
            let voxel_entity = commands.spawn((MaterialMesh2dBundle {
                mesh: meshes.add(shape::Quad::new(Vec2::new(PIXEL_SIZE as f32, PIXEL_SIZE as f32)).into()).into(),
                material: materials.add(ColorMaterial::from(SAND)),
                transform: Transform::from_translation(snaped_pos.extend(0.0)),
                ..Default::default()
            }, voxel));
        }
    }
    if keys.just_pressed(KeyCode::Space) {
        // Space was pressed
        let (mut bg_color, button, children) = buttons.single_mut();
        let mut text = texts.get_mut(children[0]).unwrap();
        if bg_color.0 == SAND {
            *bg_color = BackgroundColor::from(WATER);
            text.sections[0].value = "WATER".to_string();
        } else {
            *bg_color = BackgroundColor::from(SAND);
            text.sections[0].value = "SAND".to_string();
        }
    }
}

fn mouse_movement_updating_system(
    mut mouse_pos_screen: ResMut<MouseLocScreen>,
    mut mouse_pos_world: ResMut<MouseLocWorld>,
    mut windows: Query<&mut Window>,
    mut cursor_moved_event_reader: EventReader<CursorMoved>,
) {
    for event in cursor_moved_event_reader.iter() {
        mouse_pos_screen.0 = event.position;
        mouse_pos_world.0 = Vec2 {
            x: mouse_pos_screen.0.x,
            y: mouse_pos_screen.0.y,
        };
    }
}

enum AppState {
    InGame,
}

impl Default for AppState {
    fn default() -> Self {
        AppState::InGame
    }
}