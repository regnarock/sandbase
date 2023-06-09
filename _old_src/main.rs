mod voxels;

use bevy::{
    input::mouse::{MouseButton},
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use std::collections::HashMap;

const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 600.0;
const PIXEL_SIZE: usize = 10;
const GAME_WORLD_WIDTH: usize = (SCREEN_WIDTH / PIXEL_SIZE as f32) as usize;
const GAME_WORLD_HEIGHT: usize = (SCREEN_HEIGHT / PIXEL_SIZE as f32) as usize;
const VOXELS: usize = GAME_WORLD_WIDTH * GAME_WORLD_HEIGHT;
const SAND: Color = Color::rgb(0.761, 0.698, 0.);
const WATER: Color = Color::rgb(0., 0.749, 1.);
const EARTH: Color = Color::rgb(0.545, 0.271, 0.075);

#[derive(Component)]
#[derive(Copy, Clone, Debug, PartialEq)]
enum Voxel {
    Empty,
    Solid(VoxelData),
    Liquid(VoxelData),
}

#[derive(Component)]
#[derive(Copy, Clone, Debug, PartialEq)]
struct VoxelData {
    size: usize,
    speed: f32,
    element: Element,
    id: Entity,
}

#[derive(Component)]
#[derive(Copy, Clone, Debug, PartialEq)]
struct WorldPos {
    x: usize,
    y: usize
}

impl Voxel {
    fn update(&self, world: &mut GameWorld, current_index: usize) -> Option<Move> {
        match self {
            Voxel::of { data: VoxelStruct { element: e, .. }, .. } => match e {
                Element::Water => update_water(world, current_index),
                Element::Sand => update_sand(world, current_index),
                Element::Earth => update_earth(world, current_index),
            },
            // no-op for Out Of Bounds voxels
            Voxel::OOB => None
        }
    }
}

#[derive(Default, Resource)]
struct VoxelManager {
    sand_material: Handle<ColorMaterial>,
    water_material: Handle<ColorMaterial>,
    earth_material: Handle<ColorMaterial>,
}

impl VoxelManager {
    fn spawn_voxel(&self, element: Element) -> VoxelStruct {
        match element {
            Element::Sand => VoxelStruct {
                size: PIXEL_SIZE,
                speed: PIXEL_SIZE as f32,
                element,
                kind: Kind::Solid,
            },
            Element::Water => VoxelStruct {
                size: PIXEL_SIZE,
                speed: PIXEL_SIZE as f32,
                element,
                kind: Kind::Liquid,
            },
            Element::Earth => VoxelStruct {
                size: PIXEL_SIZE,
                speed: PIXEL_SIZE as f32,
                element,
                kind: Kind::Solid,
            },
        }
    }

    fn get_material(&self, element: Element) -> Handle<ColorMaterial> {
        match element {
            Element::Sand => self.sand_material.clone(),
            Element::Water => self.water_material.clone(),
            Element::Earth => self.earth_material.clone(),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Kind {
    Solid,
    Liquid,
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Element {
    Sand,
    Water,
    Earth,
}

enum Move {
    Displace(usize),
    Swap(usize),
}

#[derive(Resource)]
struct GameWorld {
    voxels: [Voxel; VOXELS],
    mode: Element,
}

impl Default for GameWorld {
    fn default() -> Self {
        Self {
            voxels: [Voxel::Empty; VOXELS],
            mode: Element::Sand,
        }
    }
}

#[derive(Default, Resource)]
struct VoxelMesh(Mesh2dHandle);

#[derive(Default, Resource)]
struct MouseLocScreen(Vec2);

#[derive(Default, Resource)]
struct MouseLocWorld(Vec2);

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
        .init_resource::<VoxelManager>()
        .init_resource::<VoxelMesh>()
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
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut voxel_manager: ResMut<VoxelManager>,
    mut voxel_mesh: ResMut<VoxelMesh>,
) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_translation(Vec3::new(
            SCREEN_WIDTH / 2.0,
            SCREEN_HEIGHT / 2.0,
            0.0,
        )),
        ..Default::default()
    });

    let sand_color_material = ColorMaterial::from(SAND);
    voxel_manager.sand_material = materials.add(sand_color_material);

    let water_color_material = ColorMaterial::from(WATER);
    voxel_manager.water_material = materials.add(water_color_material);

    let earth_color_material = ColorMaterial::from(EARTH);
    voxel_manager.earth_material = materials.add(earth_color_material);

    let quad = shape::Quad::new(Vec2::new(PIXEL_SIZE as f32, PIXEL_SIZE as f32));
    *voxel_mesh = VoxelMesh(meshes.add(quad.into()).into());
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

fn update_voxel_world(
    mut world: ResMut<GameWorld>,
) {
    for (mut transform, voxel) in query.iter_mut() {
        let current_index = get_index_from_pos(transform.translation.truncate());

        if current_index > GAME_WORLD_WIDTH {
            let old_voxel = world.voxels[current_index];
            match voxel.update(&mut world, current_index) {
                Some(Move::Displace(new_index)) => {
                    world.voxels[current_index] = None;
                    world.voxels[new_index] = old_voxel;
                    let new_pos = get_pos_from_index(new_index);
                    transform.translation = Vec3::new(new_pos.x, new_pos.y, 0.0);
                },
/*                Some(Move::Swap(new_index)) => {
                    let tmp_voxel = world.voxels[new_index];
                    world.voxels[new_index] = old_voxel;
                    world.voxels[current_index] = tmp_voxel;
                    let old_pos = get_pos_from_index(current_index);
                    let new_pos = get_pos_from_index(new_index);
                    transform.translation = Vec3::new(new_pos.x, new_pos.y, 0.0);
                    let Ok((mut transform_old, mut voxel)) = query.get_mut(tmp_voxel.unwrap().get_id().unwrap());
                    transform_old.translation = Vec3::new(old.x, old.y, 0.0);
                }*/
                // No-op, voxel is currently stuck
                _ => ()
            }
        }
    }
}



fn update_water(world: &mut GameWorld, current_index: usize) -> Option<Move> {
    vec![
        get_bottom_voxel(&world, current_index),
        get_bottom_left_voxel(&world, current_index),
        get_bottom_right_voxel(&world, current_index),
        get_left_voxel(&world, current_index),
        get_right_voxel(&world, current_index)
    ].iter()
        .map(|(maybe_voxel, new_index)| liquid_behaviour(maybe_voxel, *new_index))
        .find_map(|opt| opt)
}

fn update_sand(world: &mut GameWorld, current_index: usize) -> Option<Move> {
    vec![
        get_bottom_voxel(&world, current_index),
        get_bottom_left_voxel(&world, current_index),
        get_bottom_right_voxel(&world, current_index),
    ].iter()
        .map(|(maybe_voxel, new_index)| falling_solid_behaviour(maybe_voxel, *new_index))
        .find_map(|opt| opt)
}

fn update_earth(world: &mut GameWorld, current_index: usize) -> Option<Move> {
    vec![
        get_bottom_voxel(&world, current_index),
        get_bottom2_left_voxel(&world, current_index),
        get_bottom2_right_voxel(&world, current_index),
    ].iter()
        .map(|(maybe_voxel, new_index)| falling_solid_behaviour(maybe_voxel, *new_index))
        .find_map(|opt| opt)
}

fn falling_solid_behaviour(other_voxel: &Option<Voxel>, new_index: usize) -> Option<Move> {
    match other_voxel {
        None => Some(Move::Displace(new_index)),
        Some(voxel) => match voxel {
            Voxel::of { data: VoxelStruct { kind: Kind::Liquid, .. }, .. } => {
                Some(Move::Swap(new_index))
            }
            _ => None,
        }
    }
}

fn liquid_behaviour(other_voxel: &Option<Voxel>, new_index: usize) -> Option<Move> {
    match other_voxel {
        None => Some(Move::Displace(new_index)),
        Some(voxel) => match voxel {
            _ => None,
        }
    }
}

fn get_left_voxel(world: &GameWorld, index: usize) -> (Option<Voxel>, usize) {
    let left_index = index as i32 - 1;
    let maybe_voxel = if left_index >= 0 {
        world.voxels[left_index as usize]
    } else {
        Some(Voxel::OOB)
    };
    (maybe_voxel, left_index as usize)
}

fn get_bottom_voxel(world: &GameWorld, index: usize) -> (Option<Voxel>, usize) {
    let bottom_index = index - GAME_WORLD_WIDTH;
    let maybe_voxel = if bottom_index < VOXELS {
        world.voxels[bottom_index]
    } else {
        Some(Voxel::OOB)
    };
    (maybe_voxel, bottom_index)
}

fn get_right_voxel(world: &GameWorld, index: usize) -> (Option<Voxel>, usize) {
    let right_index = index + 1;
    let maybe_voxel = if right_index < VOXELS {
        world.voxels[right_index]
    } else {
        Some(Voxel::OOB)
    };
    (maybe_voxel, right_index)
}

fn get_bottom_left_voxel(world: &GameWorld, index: usize) -> (Option<Voxel>, usize) {
    let left_index = index - 1 - GAME_WORLD_WIDTH;
    let maybe_voxel = if left_index >= 0 {
        world.voxels[left_index]
    } else {
        Some(Voxel::OOB)
    };
    (maybe_voxel, left_index)
}

fn get_bottom_right_voxel(world: &GameWorld, index: usize) -> (Option<Voxel>, usize) {
    let right_index = index + 1 - GAME_WORLD_WIDTH;
    let maybe_voxel = if right_index >= 0 {
        world.voxels[right_index]
    } else {
        Some(Voxel::OOB)
    };
    (maybe_voxel, right_index)
}

fn get_bottom2_left_voxel(world: &GameWorld, index: usize) -> (Option<Voxel>, usize) {
    let left_index = index as i32 - 1 - GAME_WORLD_WIDTH as i32 * 2;
    let maybe_voxel = if left_index >= 0 {
        world.voxels[left_index as usize]
    } else {
        Some(Voxel::OOB)
    };
    (maybe_voxel, left_index as usize)
}

fn get_bottom2_right_voxel(world: &GameWorld, index: usize) -> (Option<Voxel>, usize) {
    let right_index = index as i32 + 1 - GAME_WORLD_WIDTH as i32 * 2;
    let maybe_voxel = if right_index >= 0 {
        world.voxels[right_index as usize]
    } else {
        Some(Voxel::OOB)
    };
    (maybe_voxel, right_index as usize)
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
    voxel_manager: Res<VoxelManager>,
    voxel_mesh: Res<VoxelMesh>,
    mut world: ResMut<GameWorld>,
    mouse_button_input: Res<Input<MouseButton>>,
    mut buttons: Query<(&mut BackgroundColor, &Button, &Children)>,
    mut texts: Query<&mut Text>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) || mouse_button_input.pressed(MouseButton::Left) {
        let pos = mouse_loc.0;
        let index = get_index_from_pos(pos);
        let snaped_pos = get_pos_from_index(index);
        if world.voxels[index].is_none() {
            let voxel_data = voxel_manager.spawn_voxel(world.mode);
            let mut voxel = Voxel::of { data: voxel_data, id: None };
            world.voxels[index] = Some(voxel);
            let pos = get_pos_from_index(index);
            let voxel_entity = commands.spawn((MaterialMesh2dBundle {
                mesh: voxel_mesh.0.clone(),
                material: voxel_manager.get_material(world.mode),
                transform: Transform::from_translation(snaped_pos.extend(0.0)),
                ..Default::default()
            }, voxel)).id();
            voxel.set_id(voxel_entity);
        }
    }
    if keys.just_pressed(KeyCode::Space) {
        // Space was pressed
        let (mut bg_color, button, children) = buttons.single_mut();
        let mut text = texts.get_mut(children[0]).unwrap();
        if bg_color.0 == SAND {
            *bg_color = BackgroundColor::from(WATER);
            text.sections[0].value = "WATER".to_string();
            world.mode = Element::Water;
        } else if bg_color.0 == EARTH {
            *bg_color = BackgroundColor::from(SAND);
            text.sections[0].value = "SAND".to_string();
            world.mode = Element::Sand;
        } else {
            *bg_color = BackgroundColor::from(EARTH);
            text.sections[0].value = "EARTH".to_string();
            world.mode = Element::Earth;
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
