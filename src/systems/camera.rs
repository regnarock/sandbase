use bevy::render::camera::Viewport;

use bevy::sprite::MaterialMesh2dBundle;
use bevy::{prelude::*, window::*};

use crate::components::positions::screen_position::ScreenPosition;
use crate::components::positions::world_position::WorldPosition;
use crate::resources::window::size::ScreenSize;
use crate::resources::world::config::WorldConfig;
use crate::resources::world::player_world_viewpoint::PlayerWorldViewpoint;
use crate::BACKGROUND;

pub fn handle_keyboard(
    keys: Res<Input<KeyCode>>,
    mut cameras: Query<(&mut Transform, &mut Camera, &mut WorldPosition)>,
    window_size: Res<ScreenSize>,
    world_config: Res<WorldConfig>,
    mut player_viewpoint: ResMut<PlayerWorldViewpoint>,
) {
    if keys.pressed(KeyCode::Left) || keys.pressed(KeyCode::Right) {
        let camera_speed = UVec2::new(5, 0);
        let camera_origin = ScreenPosition {
            x: window_size.width / 2.0 - world_config.px_per_voxel as f32 / 2.0,
            y: window_size.height / 2.0 - world_config.px_per_voxel as f32 / 2.0,
        };
        let (mut transform, mut camera, mut camera_world_pos) = cameras.single_mut();
        let padding = camera_speed.x; // TODO add more?
        let left_limit = 0.0;
        let right_limit = world_config.pixels_width as f32 - window_size.width;
        let normalised_main_cam_pos =
            (ScreenPosition::from_vec3(transform.translation) - camera_origin);

        if keys.pressed(KeyCode::Left) {
            player_viewpoint.x = (player_viewpoint.x - 5) % world_config.pixels_width as u32;
        }
        if keys.pressed(KeyCode::Right) {
            player_viewpoint.x = (player_viewpoint.x + 5) % world_config.pixels_width as u32;
        }

        let camera_right_side_pos =
            player_viewpoint.x as f32 + window_size.width + world_config.px_per_voxel as f32 / 2.0;
        let camera_left_side_pos = player_viewpoint.x;
        if keys.pressed(KeyCode::Left) && camera_left_side_pos > 0 {
            transform.translation -= camera_speed.extend(0).as_vec3();
        } else if keys.pressed(KeyCode::Right) {
            if camera_right_side_pos < world_config.pixels_width as f32 {
                transform.translation += camera_speed.extend(0).as_vec3();
            } else {
                println!("Moving camera viewport");
                camera.viewport.as_mut().map(|v| {
                    v.physical_size -= camera_speed * 2;
                    v.physical_position += camera_speed * 2;
                });
            }
        }

        println!(
            "{:?} <= {:?} <= {:?}",
            left_limit, player_viewpoint.x, right_limit
        );
        /*        if normalised_main_cam_pos.x < left_limit {
            camera.viewport = Some(Viewport {
                physical_position: UVec2::new(-normalised_main_cam_pos.x as u32, 0),
                physical_size: UVec2::new(
                    (world_config.pixels_width as u32 * 2 + normalised_main_cam_pos.x as u32),
                    (window_size.height * 2.0) as u32,
                ),
                ..default()
            });
        } else {*/
        /*        camera.viewport = Some(Viewport {
            physical_position: UVec2::new(0, 0),
            physical_size: UVec2::new(
                (world_config.pixels_width) as u32 * 2,
                (window_size.height) as u32 * 2,
            ),
            ..default()
        });*/
        // }

        *camera_world_pos = normalised_main_cam_pos
            .to_snapped(&world_config)
            .to_world_position(world_config.px_per_voxel);
        println!(
            "[px width={:?}] {:?} {:?}",
            world_config.pixels_width,
            normalised_main_cam_pos.x,
            camera.logical_viewport_rect().unwrap()
        );
    }
}

#[derive(Component)]
pub struct CameraWorld;

#[derive(Component)]
pub struct CameraMain;

pub fn handle_window_resize(
    mut commands: Commands,
    mut resize_reader: EventReader<WindowResized>,
    mut created_reader: EventReader<WindowCreated>,
    mut window_size: ResMut<ScreenSize>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
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
        if let Ok((mut transform, _camera, mut pos)) = cameras.get_single_mut() {
            *transform = Transform::from_translation(screen_position.to_vec3());
            *pos = WorldPosition::default();
        }
    }
    if !created_reader.is_empty() {
        let screen_position = ScreenPosition {
            x: window_size.width / 2.0 - world_config.px_per_voxel as f32 / 2.0,
            y: window_size.height / 2.0 - world_config.px_per_voxel as f32 / 2.0,
        };
        created_reader.clear();

        // main camera
        // FIXME: handle_window_resize shouldn't be responsible for spawning the camera, it's more
        // something to find in an init system
        commands.spawn((
            Camera2dBundle {
                transform: Transform::from_translation(screen_position.to_vec3()),
                ..Default::default()
            },
            WorldPosition { x: 0, y: 0 },
            CameraMain,
        ));

        let background_dimensions = Vec2::new(
            world_config.pixels_width as f32,
            world_config.pixels_height as f32,
        );
        let quad_background = shape::Quad::new(background_dimensions);
        let mesh_background = meshes.add(quad_background.into()).into();
        let background_color_material = ColorMaterial::from(BACKGROUND);
        let material_background = color_materials.add(background_color_material);

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_camera_left() {
        // Setup app
        let mut app = App::new();

        // Add Score resource
        //app.insert_resource();
        let voxels_width = 30 + 30;
        let voxels_height = 72;
        let world_config = WorldConfig::new(voxels_width, voxels_height, 10);

        // Add our two systems
        app //.add_plugins()
            .init_resource::<ScreenSize>()
            .insert_resource(world_config)
            .add_system(handle_window_resize)
            .add_system(handle_keyboard);

        // Setup test resource
        let mut input = Input::<KeyCode>::default();
        input.press(KeyCode::Left);
        app.insert_resource(input);

        // Run systems
        app.update();

        // Check resulting changes
        assert!(app
            .world
            .query::<&Camera>()
            .single(&app.world)
            .viewport
            .is_some());
        //assert_eq!(app.world.query::<&Camera>().unwrap(), 4);
    }
}
