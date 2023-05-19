use bevy::{
    prelude::*,
    window::*,
};
use crate::positions::screen_position::ScreenPosition;
use crate::{WindowSize, GameWorld, WorldConfig};

pub fn handle_window_resize(
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
        let screen_position = ScreenPosition {
            x: window_size.width / 2.0 - world_config.voxel_size / 2.0,
            y: window_size.height / 2.0 - world_config.voxel_size / 2.0,
        };
        match cameras.get_single_mut() {
            Ok((mut transform, _camera)) => {
                *transform = Transform::from_translation(screen_position.to_vec3())
            }
            _ => ()
        }
    }
    if !created_reader.is_empty() {
        let screen_position = ScreenPosition {
            x: window_size.width / 2.0 - world_config.voxel_size / 2.0,
            y: window_size.height / 2.0 - world_config.voxel_size / 2.0,
        };
        created_reader.clear();
        commands.spawn(Camera2dBundle {
            transform: Transform::from_translation(screen_position.to_vec3()),
            ..Default::default()
        });
    }
}

pub fn handle_inputs(
    keys: Res<Input<KeyCode>>,
    mut cameras: Query<(&mut Transform, &Camera)>,
    window_size: Res<WindowSize>,
    world_config: Res<WorldConfig>,
    mut world: ResMut<GameWorld>,
) {
    let camera_speed = 5.0;

    if keys.pressed(KeyCode::Left) {
// TODO wrap world in camera display
        match cameras.get_single_mut() {
            Ok((mut transform, _camera)) => {
                let padding_left = camera_speed; // TODO add more?
                let left_limit = window_size.width / 2.0 - padding_left;
                if transform.translation.x > left_limit {
                    *transform = Transform::from_translation(Vec3::new(
                        transform.translation.x - camera_speed,
                        transform.translation.y,
                        0.0,
                    ))
                }
            }
            _ => ()
        }
    } else if keys.pressed(KeyCode::Right) {
// TODO wrap world in camera display
        match cameras.get_single_mut() {
            Ok((mut transform, _camera)) => {
                println!("Camera pos: {:?}", transform.translation);
                let padding_left = camera_speed; // TODO add more?
                let right_limit = window_size.width / 2.0 + (world_config.pixels_width as f32 - window_size.width) - padding_left;
                println!("Camera should be limited at {:?}", right_limit);
                if transform.translation.x < right_limit {
                    *transform = Transform::from_translation(Vec3::new(
                        transform.translation.x + 5.0,
                        transform.translation.y,
                        0.0,
                    ))
                } else {}
            }
            _ => ()
        }
    }
}