mod voxels;
mod snapped_position;
mod screen_position;
mod world_position;

use bevy::{
    input::mouse::{MouseButton},
    prelude::*,
    window::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use crate::world_position::WorldPosition;
use crate::screen_position::ScreenPosition;
use crate::voxels::*;
use crate::snapped_position::SnappedPosition;

#[derive(Resource, Copy, Clone, Debug)]
pub struct WorldConfig {
    pub voxels_width: usize,
    pub voxels_height: usize,
    pub pixels_width: usize,
    pub pixels_height: usize,
    pub voxels: usize,
    pub pixel_size: usize,
    pub voxel_size: f32,
}

impl WorldConfig {
    fn new(voxels_width: usize, voxels_height: usize, pixel_size: usize) -> Self {
        let world_config = WorldConfig {
            voxels_width,
            voxels_height,
            pixels_width: voxels_width * pixel_size,
            pixels_height: voxels_height * pixel_size,
            voxels: voxels_width * voxels_height,
            pixel_size,
            voxel_size: pixel_size as f32,
        };
        println!("World created with config: {:?}", world_config);
        world_config
    }
}

const SAND: Color = Color::rgb(0.761, 0.698, 0.);
const WATER: Color = Color::rgb(0., 0.749, 1.);
const EARTH: Color = Color::rgb(0.545, 0.271, 0.075);


#[derive(Resource)]
pub struct GameWorld {
    voxels: Vec<Vec<Option<Voxel>>>,
    mode: Element,
}

impl GameWorld {
    pub fn new(width: usize, height: usize) -> Self {
        GameWorld {
            voxels: vec![vec![None; height]; width],
            mode: Element::Sand,
        }
    }

    pub fn get_cell(&self, wpos: WorldPosition) -> Option<Voxel> {
        self.voxels[wpos.x][wpos.y]
    }

    pub fn set_cell(&mut self, wpos: WorldPosition, voxel: &Voxel) {
        self.voxels[wpos.x][wpos.y] = Some(*voxel)
    }

    pub fn delete_cell(&mut self, wpos: WorldPosition) {
        self.voxels[wpos.x][wpos.y] = None
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
struct ScreenSize {
    width: i32,
    height: i32,
}

fn main() {
    let voxels_width = 128;
    let voxels_height = 72;
    let world_config = WorldConfig::new(voxels_width, voxels_height, 10);
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Welcome to Sandbase!".into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(world_config)
        .insert_resource(GameWorld::new(voxels_width, voxels_height))
        .init_resource::<VoxelManager>()
        .init_resource::<VoxelMesh>()
        .init_resource::<MouseLocScreen>()
        .init_resource::<MouseLocWorld>()
        .init_resource::<WindowSize>()
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
    mut cameras: Query<(&mut Transform, &Camera)>,
    world_config: Res<WorldConfig>,
) {
    for e in resize_reader.iter() {
        println!("Screen resized to {:?} {:?}", e.width, e.height);
        println!("Initial number of voxels seen height {} width {}", e.height / world_config.voxel_size, e.width / world_config.voxel_size);
        window_size.width = e.width;
        window_size.height = e.height;
        match cameras.get_single_mut() {
            Ok((mut transform, _camera)) => {
                *transform = Transform::from_translation(Vec3::new(
                    window_size.width / 2.0 - world_config.voxel_size / 2.0,
                    window_size.height / 2.0 - world_config.voxel_size / 2.0,
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
                window_size.width / 2.0 - world_config.voxel_size / 2.0,
                window_size.height / 2.0 - world_config.voxel_size / 2.0,
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
    mut world_config: Res<WorldConfig>,
    mut voxel_mesh: ResMut<VoxelMesh>,
) {
    let sand_color_material = ColorMaterial::from(SAND);
    voxel_manager.sand_material = materials.add(sand_color_material);

    let water_color_material = ColorMaterial::from(WATER);
    voxel_manager.water_material = materials.add(water_color_material);

    let earth_color_material = ColorMaterial::from(EARTH);
    voxel_manager.earth_material = materials.add(earth_color_material);

    let quad = shape::Quad::new(Vec2::new(world_config.voxel_size, world_config.voxel_size));
    *voxel_mesh = VoxelMesh(meshes.add(quad.into()).into());
}

fn setup_voxel_scene(
    mut commands: Commands,
    voxel_manager: Res<VoxelManager>,
    voxel_mesh: Res<VoxelMesh>,
    mut world: ResMut<GameWorld>,
    world_config: Res<WorldConfig>,
) {
    for x in 1..=2 {
        for y in 60..=61 {
            let world_position = WorldPosition { x, y };
            let snapped_position = world_position.to_snapped(world_config.voxel_size);
            println!("Pop block {} {} at world pos {} {}", x, y, world_position.x, world_position.y);
            let voxel_data = voxel_manager.spawn_voxel(&*world_config, Element::Sand);
            let voxel = Voxel::of { data: voxel_data };
            world.set_cell(world_position, &voxel);
            commands.spawn((MaterialMesh2dBundle {
                mesh: voxel_mesh.0.clone(),
                material: voxel_manager.get_material(voxel_data.element),
                transform: Transform::from_translation(snapped_position.to_screen_position().to_vec3()),
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
    world_config: Res<WorldConfig>,
    mut world: ResMut<GameWorld>,
    mut query: Query<(&mut Transform, &mut Voxel)>,
) {
    for (mut transform, voxel) in query.iter_mut() {
        let screen_position = ScreenPosition::from_vec2(transform.translation.truncate());
        let snapped_position = screen_position.to_snapped(&world_config);
        let world_position = snapped_position.to_world_position(world_config.voxel_size);

        match voxel.update(&*world_config, &mut world, world_position) {
            Some(Move::Displace(new_world_position)) => {
                world.delete_cell(world_position);
                world.set_cell(new_world_position, &*voxel);
                transform.translation = new_world_position.to_snapped(world_config.voxel_size)
                    .to_screen_position()
                    .to_vec3();
            }
            /*            Some(Move::Swap(new_index)) => {
                            let tmp_voxel = world.voxels[new_index];
                            world.voxels[new_index] = old_voxel;
                            world.voxels[index] = tmp_voxel;
                            let old_pos = WorldPosition::from_index(new_index, world_config.voxels_width).as_snapped(world_config.voxel_size).as_screen_position();
                            let new_pos = WorldPosition::from_index(new_index, world_config.voxels_width).as_snapped(world_config.voxel_size).as_screen_position();
                            transform.translation = Vec3::new(new_pos.x, new_pos.y, 0.0);
                            let Ok((mut transform_old, mut _voxel)) = query.get_mut(tmp_voxel.unwrap().get_id().unwrap());
                            transform_old.translation = Vec3::new(old_pos.x, old_pos.y, 0.0);
                        }*/
// No-op, voxel is currently stuck
            _ => ()
        }
    }
}

fn get_left_voxel(
    world: &GameWorld,
    world_position: WorldPosition,
    world_config: &WorldConfig,
) -> (Option<Voxel>, WorldPosition) {
    let wrapped_x = if world_position.x >= 1 {
        world_position.x - 1
    } else {
        world_config.voxels_width - 1
    };
    let left_pos = WorldPosition {
        x: wrapped_x,
        y: world_position.y,
    };
    let maybe_voxel = world.get_cell(left_pos);
    (maybe_voxel, left_pos)
}

fn get_bottom_voxel(
    world: &GameWorld,
    world_position: WorldPosition,
    _world_config: &WorldConfig,
) -> (Option<Voxel>, WorldPosition) {
    if world_position.y < 1 {
        return (Some(Voxel::OOB), world_position)
    }
    let bottom_pos = WorldPosition {
        x: world_position.x,
        y: world_position.y - 1,
    };
    (world.get_cell(bottom_pos), bottom_pos)
}

fn get_right_voxel(
    world: &GameWorld,
    world_position: WorldPosition,
    world_config: &WorldConfig,
) -> (Option<Voxel>, WorldPosition) {
    let wrapped_x = if world_position.x < world_config.voxels_width - 1 {
        world_position.x + 1
    } else {
        0
    };
    let right_pos = WorldPosition {
        x: wrapped_x,
        y: world_position.y,
    };
    let maybe_voxel = world.get_cell(right_pos);
    (maybe_voxel, right_pos)
}

fn get_bottom_left_voxel(
    world: &GameWorld,
    world_position: WorldPosition,
    world_config: &WorldConfig,
) -> (Option<Voxel>, WorldPosition) {
    if world_position.y < 1 {
        return (Some(Voxel::OOB), world_position)
    }

    let wrapped_x = if world_position.x >= 1 {
        world_position.x - 1
    } else {
        world_config.voxels_width - 1
    };
    let left_bottom_pos = WorldPosition {
        x: wrapped_x,
        y: world_position.y - 1,
    };
    let maybe_voxel = world.get_cell(left_bottom_pos);
    (maybe_voxel, left_bottom_pos)
}

fn get_bottom_right_voxel(
    world: &GameWorld,
    world_position: WorldPosition,
    world_config: &WorldConfig,
) -> (Option<Voxel>, WorldPosition) {
    if world_position.y < 1 {
        return (Some(Voxel::OOB), world_position)
    }
    let wrapped_x = if world_position.x < world_config.voxels_width - 1 {
        world_position.x + 1
    } else {
        0
    };
    let right_bottom_pos = WorldPosition {
        x: wrapped_x,
        y: world_position.y - 1,
    };
    let maybe_voxel = world.get_cell(right_bottom_pos);
    (maybe_voxel, right_bottom_pos)
}

fn get_bottom2_left_voxel(
    world: &GameWorld,
    world_position: WorldPosition,
    world_config: &WorldConfig,
) -> (Option<Voxel>, WorldPosition) {
    if world_position.y < 2 {
        return (Some(Voxel::OOB), world_position)
    }
    let wrapped_x = if world_position.x >= 1 {
        world_position.x - 1
    } else {
        world_config.voxels_width - 1
    };
    let left_bottom_pos = WorldPosition {
        x: wrapped_x,
        y: world_position.y - 2,
    };
    let maybe_voxel = world.get_cell(left_bottom_pos);
    (maybe_voxel, left_bottom_pos)
}

fn get_bottom2_right_voxel(
    world: &GameWorld,
    world_position: WorldPosition,
    world_config: &WorldConfig,
) -> (Option<Voxel>, WorldPosition) {
    if world_position.y < 2 {
        return (Some(Voxel::OOB), world_position)
    }

    let wrapped_x = if world_position.x < world_config.voxels_width - 1 {
        world_position.x + 1
    } else {
        0
    };
    let right_bottom_pos = WorldPosition {
        x: wrapped_x,
        y: world_position.y - 2,
    };
    let maybe_voxel = world.get_cell(right_bottom_pos);
    (maybe_voxel, right_bottom_pos)
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
    world_config: Res<WorldConfig>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) || mouse_button_input.pressed(MouseButton::Left) {
        let snapped_pos: SnappedPosition = mouse_loc.0.to_snapped(&world_config);
        let world_position = snapped_pos.to_world_position(world_config.voxel_size);

        if world.get_cell(world_position).is_none() {
            let voxel_data = voxel_manager.spawn_voxel(&*world_config, world.mode);
            let voxel = Voxel::of { data: voxel_data };
            world.set_cell(world_position, &voxel);
            commands.spawn((MaterialMesh2dBundle {
                mesh: voxel_mesh.0.clone(),
                material: voxel_manager.get_material(world.mode),
                transform: Transform::from_translation(snapped_pos.to_screen_position().to_vec3()),
                ..Default::default()
            }, voxel));
        }
    }
    if keys.just_pressed(KeyCode::Space) {
// Space was pressed
        let (mut bg_color, _button, children) = buttons.single_mut();
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
    if keys.pressed(KeyCode::Left) {
// TODO wrap world in camera display
        match cameras.get_single_mut() {
            Ok((mut transform, _camera)) => {
                *transform = Transform::from_translation(Vec3::new(
                    transform.translation.x - 5.0,
                    transform.translation.y,
                    0.0,
                ))
            }
            _ => ()
        }
    } else if keys.pressed(KeyCode::Right) {
// TODO wrap world in camera display
        match cameras.get_single_mut() {
            Ok((mut transform, _camera)) => {
                *transform = Transform::from_translation(Vec3::new(
                    transform.translation.x + 5.0,
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
        let world_config = WorldConfig::new(256, 144, 10);

        let screen_position = ScreenPosition {
            x: 640.0,
            y: 360.0,
        };

        let world_position = screen_position.to_snapped(world_config.voxel_size).to_world_position(world_config.voxel_size);
        let index = world_position.as_index(world_config.voxels_width);
        let snapped_position = world_position.to_snapped(world_config.voxel_size);

        assert_eq!(world_position.x, 64); // 640/10
        assert_eq!(world_position.y, 36); // 360/10
        assert_eq!(snapped_position.x, 640.0);
        assert_eq!(snapped_position.y, 360.0);
    }

    #[test]
    fn world_position_should_be_x_11_and_y_42_and_index_should_be_10763() {
        let world_config = WorldConfig::new(10, 144, 10);

        let screen_size = ScreenSize {
            width: 1280,
            height: 720,
        };
        let screen_position = ScreenPosition {
            x: 117.0,
            y: 429.0,
        };

        let world_position = screen_position.to_snapped(world_config.voxel_size).to_world_position(world_config.voxel_size);
        let new_world_position = WorldPosition::from_index(index, world_config.voxels_width);
        let snapped_position = world_position.to_snapped(world_config.voxel_size);

        assert_eq!(world_position.x, 11); // 117/10
        assert_eq!(world_position.y, 42); // 429/10
        assert_eq!(new_world_position.x, 11); // 117/10
        assert_eq!(new_world_position.y, 42); // 429/10
        assert_eq!(snapped_position.x, 110.0);
        assert_eq!(snapped_position.y, 420.0);
    }

    #[test]
    fn test() {
        let screen_size = ScreenSize {
            width: 1280,
            height: 720,
        };
        let screen_position = ScreenPosition {
            x: 117.0,
            y: 429.0,
        };
        let world_config = WorldConfig::new(10, 144, 10);
        let world_position = screen_position.to_snapped(world_config.voxel_size).to_world_position(world_config.voxel_size);
    }

    #[test]
    fn test2() {
        const WORLD_VOXEL_WIDTH: usize = 10;
        const WORLD_VOXEL_HEIGHT: usize = 144;
        const VOXELS: usize = WORLD_VOXEL_WIDTH * WORLD_VOXEL_HEIGHT;
        const PIXEL_SIZE: usize = 10;
        const VOXEL_SIZE: f32 = 10.0;
        const WORLD_PIXEL_WIDTH: usize = WORLD_VOXEL_WIDTH * PIXEL_SIZE;
        const WORLD_PIXEL_HEIGHT: usize = WORLD_VOXEL_HEIGHT * PIXEL_SIZE;

        let config = WorldConfig::new(WORLD_VOXEL_WIDTH, WORLD_VOXEL_HEIGHT, PIXEL_SIZE);

        assert_eq!(WORLD_VOXEL_WIDTH, config.voxels_width);
        assert_eq!(WORLD_VOXEL_HEIGHT, config.voxels_height);
        assert_eq!(VOXELS, config.voxels);
        assert_eq!(PIXEL_SIZE, config.pixel_size);
        assert_eq!(VOXEL_SIZE, config.voxel_size);
        assert_eq!(WORLD_PIXEL_WIDTH, config.pixels_width);
        assert_eq!(WORLD_PIXEL_HEIGHT, config.pixels_height);
    }
}