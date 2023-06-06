use bevy::input::InputPlugin;
use bevy::prelude::*;
use bevy::utils::tracing::instrument::WithSubscriber;

use components::voxels::*;

use crate::components::positions::screen_position::ScreenPosition;
use crate::plugins::inputs::cursor::CursorPlugin;
use crate::plugins::inputs::InputsPluginGroup;
use crate::resources::voxels::default_mesh::VoxelMesh;
use crate::resources::window::size::ScreenSize;
use crate::resources::world::config::WorldConfig;
use crate::resources::world::map::GameMap;
use crate::systems::inputs::{game_cursor, keyboard};
use crate::systems::{camera, startup};

mod components;
mod plugins;
mod resources;
mod systems;

const BACKGROUND: Color = Color::rgb(0., 0., 0.);
const WATER: Color = Color::rgb(0., 0.749, 1.);
const SAND: Color = Color::rgb(0.761, 0.698, 0.);
const EARTH: Color = Color::rgb(0.545, 0.271, 0.075);

fn main() {
    let voxels_width = 128 + 30;
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
        .add_plugins(InputsPluginGroup)
        .insert_resource(world_config)
        .insert_resource(GameMap::new(voxels_width, voxels_height))
        .init_resource::<VoxelManager>()
        .init_resource::<VoxelMesh>()
        .init_resource::<ScreenSize>()
        .add_system(camera::handle_window_resize)
        .add_system(camera::handle_keyboard)
        .add_startup_system(startup::setup)
        .add_startup_system(startup::setup_ui)
        .add_startup_system(game_cursor::setup_voxel_scene)
        .add_system(game_cursor::handle_cursor_moved)
        .add_system(game_cursor::handle_camera_move)
        // TODO: make it one function
        .add_system(game_cursor::handle_button)
        .add_system(keyboard::handle_input)
        .add_system(update_voxel_world)
        .run();
}

fn update_voxel_world(
    world_config: Res<WorldConfig>,
    screen_size: Res<ScreenSize>,
    mut map: ResMut<GameMap>,
    mut query: Query<(&mut Transform, &mut Voxel)>,
) {
    for (mut transform, voxel) in query.iter_mut() {
        let screen_position = ScreenPosition::from_vec2(transform.translation.truncate());
        let snapped_position = screen_position.to_snapped(&world_config);
        let world_position = snapped_position.to_world_position(world_config.px_per_voxel);

        match voxel.update(&*world_config, &mut map, world_position) {
            Some(Move::Displace(new_world_position)) => {
                map.delete_cell(&world_position);
                map.set_cell(&new_world_position, &*voxel);
                transform.translation = new_world_position
                    .to_snapped(world_config.px_per_voxel)
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
            _ => (),
        }
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
