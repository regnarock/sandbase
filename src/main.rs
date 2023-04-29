mod voxels;

use bevy::{
    input::mouse::{MouseButton},
    prelude::*,
    window::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use std::collections::HashMap;

// TODO: to be configurable
const WORLD_VOXEL_WIDTH: usize = 10;
const WORLD_VOXEL_HEIGHT: usize = 144;
const VOXELS: usize = WORLD_VOXEL_WIDTH * WORLD_VOXEL_HEIGHT;
const PIXEL_SIZE: usize = 10;
const VOXEL_SIZE: f32 = 10.0;
const WORLD_PIXEL_WIDTH: usize = WORLD_VOXEL_WIDTH * PIXEL_SIZE;
const WORLD_PIXEL_HEIGHT: usize = WORLD_VOXEL_HEIGHT * PIXEL_SIZE;

const SAND: Color = Color::rgb(0.761, 0.698, 0.);
const WATER: Color = Color::rgb(0., 0.749, 1.);
const EARTH: Color = Color::rgb(0.545, 0.271, 0.075);

#[derive(Component)]
#[derive(Copy, Clone, Debug, PartialEq)]
enum Voxel {
    OOB,
    of { data: VoxelStruct },
}

#[derive(Component)]
#[derive(Copy, Clone, Debug, PartialEq)]
struct VoxelStruct {
    size: usize,
    speed: f32,
    element: Element,
    kind: Kind,
}

#[derive(Component)]
#[derive(Default, Copy, Clone, Debug, PartialEq)]
struct WorldPos {
    x: usize,
    y: usize,
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
                speed: VOXEL_SIZE,
                element,
                kind: Kind::Solid,
            },
            Element::Water => VoxelStruct {
                size: PIXEL_SIZE,
                speed: VOXEL_SIZE,
                element,
                kind: Kind::Liquid,
            },
            Element::Earth => VoxelStruct {
                size: PIXEL_SIZE,
                speed: VOXEL_SIZE,
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
    voxels: [Option<Voxel>; VOXELS],
    mode: Element,
}

impl Default for GameWorld {
    fn default() -> Self {
        Self {
            voxels: [None; VOXELS],
            mode: Element::Sand,
        }
    }
}

#[derive(Default, Resource)]
struct VoxelMesh(Mesh2dHandle);

#[derive(Default, Resource)]
struct MouseLocScreen(Vec2);

#[derive(Default, Resource)]
struct MouseLocWorld(ScreenPosition);

impl MouseLocWorld {
    fn from_vec2(v: Vec2) -> Self {
        MouseLocWorld(ScreenPosition {
            x: v.x,
            y: v.y,
        })
    }
}

#[derive(Default, Resource)]
struct WindowSize {
    width: f32,
    height: f32,
}

#[derive(Default, Resource, Copy, Clone, Debug)]
struct ScaleFactor {
    height: f32,
    width: f32,
}

#[derive(Default, Resource, Copy, Clone, Debug)]
struct WorldPosition {
    x: usize,
    y: usize,
}

impl WorldPosition {
    fn from_index(
        index: usize,
    ) -> WorldPosition {
        WorldPosition {
            x: index % WORLD_VOXEL_WIDTH,
            y: index / WORLD_VOXEL_WIDTH,
        }
    }

    fn as_index(&self) -> usize {
        self.y * WORLD_VOXEL_WIDTH + self.x
    }

    fn as_snapped(&self) -> SnappedPosition {
        SnappedPosition {
            x: self.x as f32 * VOXEL_SIZE,
            y: self.y as f32 * VOXEL_SIZE,
        }
    }
}

#[derive(Default, Resource, Copy, Clone, Debug)]
struct ScreenPosition {
    x: f32,
    y: f32,
}

impl ScreenPosition {
    fn from_vec2(v: Vec2) -> Self {
        ScreenPosition {
            x: v.x,
            y: v.y,
        }
    }
}

#[derive(Default, Resource, Copy, Clone, Debug)]
struct SnappedPosition {
    x: f32,
    y: f32,
}

impl SnappedPosition {
    fn as_vec2(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }

    fn as_screen_position(&self) -> ScreenPosition {
        ScreenPosition {
            x: self.x,
            y: self.y,
        }
    }
}

fn as_world_position(vector: Vec2) -> WorldPosition {
    WorldPosition {
        x: vector.x as usize,
        y: vector.y as usize,
    }
}

#[derive(Default, Resource, Copy, Clone, Debug)]
struct ScreenSize {
    width: i32,
    height: i32,
}

fn get_world_position_from_screen_position(
    screen_position: ScreenPosition
) -> WorldPosition {
    // TODO: add camera shift

    let x = screen_position.x;
    let y = screen_position.y;
    WorldPosition {
        x: (x / VOXEL_SIZE) as usize,
        y: (y / VOXEL_SIZE) as usize,
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Welcome to Sandbase!".into(),
                ..default()
            }),
            ..default()
        }))
        .init_resource::<VoxelManager>()
        .init_resource::<VoxelMesh>()
        .init_resource::<MouseLocScreen>()
        .init_resource::<MouseLocWorld>()
        .init_resource::<GameWorld>()
        .init_resource::<WindowSize>()
        .init_resource::<ScaleFactor>()
        .init_resource::<ScreenSize>()
        .add_system(handle_window_resize)
        .add_startup_system(setup)
        .add_startup_system(setup_ui)
        .add_startup_system(setup_voxel_scene)
        .add_system(mouse_movement_updating_system)
        .add_system(handle_input)
        .add_system(update_voxel_world)
        .run();
}

fn handle_window_resize(
    mut commands: Commands,
    mut resize_reader: EventReader<WindowResized>,
    mut created_reader: EventReader<WindowCreated>,
    mut window_size: ResMut<WindowSize>,
    mut scale_factor: ResMut<ScaleFactor>,
    mut cameras: Query<(&mut Transform, &Camera)>,
) {
    for e in resize_reader.iter() {
        println!("Screen resized to {:?} {:?}", e.width, e.height);
        println!("Initial number of voxels seen height {} width {}", e.height / VOXEL_SIZE, e.width / VOXEL_SIZE);
        window_size.width = e.width;
        window_size.height = e.height;
        scale_factor.width = window_size.width / e.width;
        scale_factor.height = window_size.height / e.height;
        println!("Initial scale factor h{} w{}", scale_factor.height, scale_factor.width);
        match cameras.get_single_mut() {
            Ok((mut transform, camera)) => {
                *transform = Transform::from_translation(Vec3::new(
                    window_size.width / 2.0 - VOXEL_SIZE / 2.0,
                    window_size.height / 2.0 - VOXEL_SIZE / 2.0,
                    0.0,
                ))
            }
            _ => ()
        }
    }
    if !created_reader.is_empty() {
        created_reader.clear();
        commands.spawn(Camera2dBundle {
            transform: Transform::from_translation(Vec3::new(
                window_size.width / 2.0 - VOXEL_SIZE / 2.0,
                window_size.height / 2.0 - VOXEL_SIZE / 2.0,
                0.0,
            )),
            ..Default::default()
        });
    }
}

fn setup(
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut voxel_manager: ResMut<VoxelManager>,
    mut voxel_mesh: ResMut<VoxelMesh>,
) {
    let sand_color_material = ColorMaterial::from(SAND);
    voxel_manager.sand_material = materials.add(sand_color_material);

    let water_color_material = ColorMaterial::from(WATER);
    voxel_manager.water_material = materials.add(water_color_material);

    let earth_color_material = ColorMaterial::from(EARTH);
    voxel_manager.earth_material = materials.add(earth_color_material);

    let quad = shape::Quad::new(Vec2::new(VOXEL_SIZE, VOXEL_SIZE));
    *voxel_mesh = VoxelMesh(meshes.add(quad.into()).into());
}

fn setup_voxel_scene(
    mut commands: Commands,
    voxel_manager: Res<VoxelManager>,
    voxel_mesh: Res<VoxelMesh>,
    mut world: ResMut<GameWorld>,
) {
    for x in 1..=2 {
        for y in 60..=61 {
            let world_position = WorldPosition { x, y };
            let index = world_position.as_index();
            let snapped_position = world_position.as_snapped();
            println!("Pop block {} {} at world pos {} {}", x, y, snapped_position.x, snapped_position.y);
            let voxel_data = voxel_manager.spawn_voxel(Element::Sand);
            let mut voxel = Voxel::of { data: voxel_data };
            world.voxels[index] = Some(voxel);
            commands.spawn((MaterialMesh2dBundle {
                mesh: voxel_mesh.0.clone(),
                material: voxel_manager.get_material(voxel_data.element),
                transform: Transform::from_translation(snapped_position.as_vec2().extend(0.0)),
                ..Default::default()
            }, voxel));
        }
    }
    /*for x in 30..=50 {
        for y in 120..=140 {
            let world_position = WorldPosition { x, y };
            let index = world_position.as_index();
            let snapped_position = world_position.as_snapped();
            println!("Pop block {} {} at world pos {} {}", x, y, snapped_position.x, snapped_position.y);
            let voxel_data = voxel_manager.spawn_voxel(Element::Water);
            let mut voxel = Voxel::of { data: voxel_data };
            world.voxels[index] = Some(voxel);
            commands.spawn((MaterialMesh2dBundle {
                mesh: voxel_mesh.0.clone(),
                material: voxel_manager.get_material(voxel_data.element),
                transform: Transform::from_translation(snapped_position.as_vec2().extend(0.0)),
                ..Default::default()
            }, voxel));
        }
    }*/
}

// Spawns the UI
fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Node that fills entire background
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::width(Val::Percent(100.0)),
                align_items: AlignItems::Start,
                justify_content: JustifyContent::Start,
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
                            left: Val::Px(0.0),
                            top: Val::Px(0.0),
                            ..default()
                        },
                        margin: UiRect {
                            left: Val::Px(0.),
                            top: Val::Px(0.),
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
    mut commands: Commands,
    mut world: ResMut<GameWorld>,
    scale_factor: Res<ScaleFactor>,
    mut query: Query<(&mut Transform, &mut Voxel)>,
) {
    let factor = scale_factor.into_inner();
    for (mut transform, voxel) in query.iter_mut() {
        let screen_position = ScreenPosition::from_vec2(transform.translation.truncate());
        let world_position = get_world_position_from_screen_position(screen_position);
        let index = world_position.as_index();

        // voxels stop being updated before falling from the screen
        let old_voxel = world.voxels[index];
        match voxel.update(&mut world, index) {
            Some(Move::Displace(new_index)) => {
                world.voxels[index] = None;
                world.voxels[new_index] = old_voxel;
                let new_pos = WorldPosition::from_index(new_index).as_snapped().as_screen_position();
                transform.translation = Vec3::new(new_pos.x, new_pos.y, 0.0);
            }
            /*                Some(Move::Swap(new_index)) => {
                            let tmp_voxel = world.voxels[new_index];
                            world.voxels[new_index] = old_voxel;
                            world.voxels[index] = tmp_voxel;
                            let old_pos = get_pos_from_index(index);
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
    let bottom_index = index as i32 - WORLD_VOXEL_WIDTH as i32;
    let maybe_voxel = if bottom_index >= 0 {
        world.voxels[bottom_index as usize]
    } else {
        Some(Voxel::OOB)
    };
    (maybe_voxel, bottom_index as usize)
}

fn get_right_voxel(world: &GameWorld, index: usize) -> (Option<Voxel>, usize) {
    let right_index = index + 1;
    let maybe_voxel = if right_index < VOXELS {
        world.voxels[right_index as usize]
    } else {
        Some(Voxel::OOB)
    };
    (maybe_voxel, right_index as usize)
}

fn get_bottom_left_voxel(world: &GameWorld, index: usize) -> (Option<Voxel>, usize) {
    let left_index = index as i32 - 1 - WORLD_VOXEL_WIDTH as i32;
    let maybe_voxel = if left_index >= 0 {
        world.voxels[left_index as usize]
    } else {
        Some(Voxel::OOB)
    };
    (maybe_voxel, left_index as usize)
}

fn get_bottom_right_voxel(world: &GameWorld, index: usize) -> (Option<Voxel>, usize) {
    let right_index = index as i32 + 1 - WORLD_VOXEL_WIDTH as i32;
    let maybe_voxel = if right_index >= 0 {
        world.voxels[right_index as usize]
    } else {
        Some(Voxel::OOB)
    };
    (maybe_voxel, right_index as usize)
}

fn get_bottom2_left_voxel(world: &GameWorld, index: usize) -> (Option<Voxel>, usize) {
    let left_index = index as i32 - 1 - WORLD_VOXEL_WIDTH as i32 * 2;
    let maybe_voxel = if left_index >= 0 {
        world.voxels[left_index as usize]
    } else {
        Some(Voxel::OOB)
    };
    (maybe_voxel, left_index as usize)
}

fn get_bottom2_right_voxel(world: &GameWorld, index: usize) -> (Option<Voxel>, usize) {
    let right_index = index as i32 + 1 - WORLD_VOXEL_WIDTH as i32 * 2;
    let maybe_voxel = if right_index >= 0 {
        world.voxels[right_index as usize]
    } else {
        Some(Voxel::OOB)
    };
    (maybe_voxel, right_index as usize)
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
    mut cameras: Query<(&mut Transform, &Camera)>,
    scale_factor: Res<ScaleFactor>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) || mouse_button_input.pressed(MouseButton::Left) {
        let factor = scale_factor.into_inner();

        let world_position = get_world_position_from_screen_position(mouse_loc.0);

        let index = world_position.as_index();
        let snapped_pos = world_position.as_snapped();

        if world.voxels[index].is_none() {
            let voxel_data = voxel_manager.spawn_voxel(world.mode);
            let mut voxel = Voxel::of { data: voxel_data };
            world.voxels[index] = Some(voxel);
            commands.spawn((MaterialMesh2dBundle {
                mesh: voxel_mesh.0.clone(),
                material: voxel_manager.get_material(world.mode),
                transform: Transform::from_translation(snapped_pos.as_vec2().extend(0.0)),
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
    if keys.just_pressed(KeyCode::Left) {
        // TODO wrap world in camera display
        match cameras.get_single_mut() {
            Ok((mut transform, camera)) => {
                *transform = Transform::from_translation(Vec3::new(
                    transform.translation.x + 5.0,
                    transform.translation.y,
                    0.0,
                ))
            }
            _ => ()
        }
    } else if keys.just_pressed(KeyCode::Right) {
        // TODO wrap world in camera display
        match cameras.get_single_mut() {
            Ok((mut transform, camera)) => {
                *transform = Transform::from_translation(Vec3::new(
                    transform.translation.x - 5.0,
                    transform.translation.y,
                    0.0,
                ))
            }
            _ => ()
        }
    }
}

fn mouse_movement_updating_system(
    mut mouse_pos_screen: ResMut<MouseLocScreen>,
    mut mouse_pos_world: ResMut<MouseLocWorld>,
    mut cursor_moved_event_reader: EventReader<CursorMoved>,
) {
    for event in cursor_moved_event_reader.iter() {
        mouse_pos_screen.0 = event.position;
        *mouse_pos_world = MouseLocWorld::from_vec2(mouse_pos_screen.0);
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

#[cfg(test)]
mod tests {
    use super::*;

    // const WORLD_VOXEL_WIDTH: usize = 256;
    // const WORLD_VOXEL_HEIGHT: usize = 144;
    // const PIXEL_SIZE: usize = 10;

    #[test]
    fn world_position_should_be_x_64_and_y_36_and_index_should_be_9280() {
        let mut screen_size = ScreenSize {
            width: 1280,
            height: 720,
        };
        let mut screen_position = ScreenPosition {
            x: 640.0,
            y: 360.0,
        };

        let mut world_position = get_world_position_from_screen_position(screen_position);
        let mut index = world_position.as_index();
        let mut snapped_position = world_position.as_snapped();

        assert_eq!(world_position.x, 64); // 640/10
        assert_eq!(world_position.y, 36); // 360/10
        assert_eq!(index, 9280); // 256*36+64
        assert_eq!(snapped_position.x, 640.0);
        assert_eq!(snapped_position.y, 360.0);
    }

    #[test]
    fn world_position_should_be_x_11_and_y_42_and_index_should_be_10763() {
        let mut screen_size = ScreenSize {
            width: 1280,
            height: 720,
        };
        let mut screen_position = ScreenPosition {
            x: 117.0,
            y: 429.0,
        };

        let mut world_position = get_world_position_from_screen_position(screen_position);
        let mut index = world_position.as_index();
        let mut new_world_position = WorldPosition::from_index(index);
        let mut snapped_position = world_position.as_snapped();

        assert_eq!(world_position.x, 11); // 117/10
        assert_eq!(world_position.y, 42); // 429/10
        assert_eq!(new_world_position.x, 11); // 117/10
        assert_eq!(new_world_position.y, 42); // 429/10
        assert_eq!(index, 10763); // 256*42+11
        assert_eq!(snapped_position.x, 110.0);
        assert_eq!(snapped_position.y, 420.0);
    }
}