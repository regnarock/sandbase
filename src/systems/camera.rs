use crate::BACKGROUND;
use bevy::math::Vec3Swizzles;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::{prelude::*, window::*};

use crate::components::positions::screen_position::ScreenPosition;
use crate::components::positions::world_position::WorldPosition;
use crate::resources::window::size::ScreenSize;
use crate::resources::world::config::WorldConfig;

pub fn handle_keyboard(
    keys: Res<Input<KeyCode>>,
    mut cameras: Query<(&mut Transform, &Camera, &mut WorldPosition)>,
    window_size: Res<ScreenSize>,
    world_config: Res<WorldConfig>,
) {
    let camera_speed = Vec3::new(5.0, 0.0, 0.0);
    let camera_origin = ScreenPosition {
        x: window_size.width / 2.0 - world_config.px_per_voxel as f32 / 2.0,
        y: window_size.height / 2.0 - world_config.px_per_voxel as f32 / 2.0,
    };

    if keys.pressed(KeyCode::Left) {
        // TODO wrap world in camera display
        let (mut transform, _, mut camera_world_pos) = cameras.single_mut();
        let padding_left = camera_speed.x; // TODO add more?
        let left_limit = window_size.width / 2.0 - padding_left;
        let shifted_screen_position =
            (ScreenPosition::from_vec3(transform.translation) - camera_origin);

        if transform.translation.x > left_limit {
            transform.translation = transform.translation - camera_speed;
            *camera_world_pos = shifted_screen_position
                .to_snapped(&world_config)
                .to_world_position(world_config.px_per_voxel);
        }
    } else if keys.pressed(KeyCode::Right) {
        // TODO wrap world in camera display
        let (mut transform, _, mut camera_world_pos) = cameras.single_mut();
        let padding_left = camera_speed.x; // TODO add more?
        let right_limit = window_size.width / 2.0
            + (world_config.pixels_width as f32 - window_size.width)
            - padding_left;
        if transform.translation.x < right_limit {
            transform.translation = transform.translation + camera_speed;
            let normalised_screen_pos =
                (ScreenPosition::from_vec3(transform.translation) - camera_origin);
            *camera_world_pos = normalised_screen_pos
                .to_snapped(&world_config)
                .to_world_position(world_config.px_per_voxel);
        }
    }
}

pub fn handle_window_resize(
    mut commands: Commands,
    mut resize_reader: EventReader<WindowResized>,
    mut created_reader: EventReader<WindowCreated>,
    mut window_size: ResMut<ScreenSize>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut cameras: Query<(&mut Transform, &Camera, &mut WorldPosition)>,
    world_config: Res<WorldConfig>,
) {
    for e in resize_reader.iter() {
        println!("Screen resized to {:?} {:?}", e.width, e.height);
        println!(
            "Initial number of voxels seen height {} width {}",
            e.height / world_config.px_per_voxel as f32,
            e.width / world_config.px_per_voxel as f32
        );
        window_size.width = e.width;
        window_size.height = e.height;
        let screen_position = ScreenPosition {
            x: window_size.width / 2.0 - world_config.px_per_voxel as f32 / 2.0,
            y: window_size.height / 2.0 - world_config.px_per_voxel as f32 / 2.0,
        };
        match cameras.get_single_mut() {
            Ok((mut transform, _camera, mut pos)) => {
                *transform = Transform::from_translation(screen_position.to_vec3());
                *pos = WorldPosition::default();
            }
            _ => (),
        }
    }
    if !created_reader.is_empty() {
        let screen_position = ScreenPosition {
            x: window_size.width / 2.0 - world_config.px_per_voxel as f32 / 2.0,
            y: window_size.height / 2.0 - world_config.px_per_voxel as f32 / 2.0,
        };
        created_reader.clear();
        commands.spawn((
            Camera2dBundle {
                transform: Transform::from_translation(screen_position.to_vec3()),
                ..Default::default()
            },
            WorldPosition { x: 0, y: 0 },
        ));

        let background_dimensions = Vec2::new(
            world_config.pixels_width as f32,
            world_config.pixels_height as f32,
        );
        let quad_background = shape::Quad::new(background_dimensions);
        let mesh_background = meshes.add(quad_background.into()).into();
        let background_color_material = ColorMaterial::from(BACKGROUND);
        let material_background = materials.add(background_color_material);

        let background_position = Vec3::new(
            (background_dimensions.x - world_config.px_per_voxel as f32) / 2.0,
            (background_dimensions.y - world_config.px_per_voxel as f32) / 2.0,
            -10.0,
        );
        println!(
            "background created with dimension: {:?} and position: {:?}",
            background_dimensions, background_position
        );
        commands.spawn(MaterialMesh2dBundle {
            mesh: mesh_background,
            material: material_background,
            transform: Transform::from_translation(background_position),
            ..Default::default()
        });
    }
}
