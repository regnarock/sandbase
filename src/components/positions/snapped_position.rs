use crate::components::positions::screen_position::ScreenPosition;
use crate::components::positions::world_position::WorldPosition;
use crate::resources::world::config::WorldConfig;
use bevy::prelude::*;

#[derive(Default, Resource, Copy, Clone, Debug)]
pub struct SnappedPosition {
    x: f32,
    y: f32,
}

impl SnappedPosition {
    pub fn to_screen_position(&self) -> ScreenPosition {
        ScreenPosition {
            x: self.x,
            y: self.y,
        }
    }

    pub fn to_world_position(&self, px_per_voxel: usize) -> WorldPosition {
        WorldPosition {
            x: (self.x as usize / px_per_voxel),
            y: (self.y as usize / px_per_voxel),
        }
    }

    pub fn from_world_position(world_pos: &WorldPosition, px_per_voxel: usize) -> SnappedPosition {
        SnappedPosition {
            x: (world_pos.x * px_per_voxel) as f32,
            y: (world_pos.y * px_per_voxel) as f32,
        }
    }

    pub fn from_screen_position(
        screen_pos: &ScreenPosition,
        world_config: &WorldConfig,
    ) -> SnappedPosition {
        let snapped_x =
            (screen_pos.x as usize / world_config.px_per_voxel) * world_config.px_per_voxel;
        let snapped_y =
            (screen_pos.y as usize / world_config.px_per_voxel) * world_config.px_per_voxel;
        SnappedPosition {
            x: snapped_x
                .min(world_config.pixels_width - world_config.px_per_voxel)
                .max(0) as f32,
            y: snapped_y
                .min(world_config.pixels_height - world_config.px_per_voxel)
                .max(0) as f32,
        }
    }
}
/*
#[cfg(test)]
mod tests {
    use super::*;
    use crate::resources::window::size::ScreenSize;

    #[test]
    fn screen_position_outside_world_gets_snapped_to_fit_in_world() {
        let world_config = WorldConfig::new(10, 144, 10);

        let screen_position = ScreenPosition { x: 640.0, y: 360.0 };

        let world_position = screen_position
            .to_snapped(&world_config)
            .to_world_position(world_config.voxel_size);
        let index = world_position.as_index(world_config.voxels_width);
        let snapped_position = world_position.to_snapped(world_config.voxel_size);

        assert_eq!(world_position.x, 64); // 640/10
        assert_eq!(world_position.y, 36); // 360/10
        assert_eq!(index, 9280); // 256*36+64
        assert_eq!(snapped_position.x, 640.0);
        assert_eq!(snapped_position.y, 360.0);
    }

    #[test]
    fn world_position_should_be_x_64_and_y_36_and_index_should_be_9280() {
        let world_config = WorldConfig::new(256, 144, 10);

        let screen_position = ScreenPosition { x: 640.0, y: 360.0 };

        let world_position = screen_position
            .to_snapped(world_config.voxel_size)
            .to_world_position(world_config.voxel_size);
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
            width: 1280 as f32,
            height: 720 as f32,
        };
        let screen_position = ScreenPosition { x: 117.0, y: 429.0 };

        let world_position = screen_position
            .to_snapped(world_config.voxel_size)
            .to_world_position(world_config.voxel_size);
        let new_world_position = WorldPosition::from_index(index, world_config.voxels_width);
        let snapped_position = world_position.to_snapped(world_config.voxel_size);

        assert_eq!(world_position.x, 11); // 117/10
        assert_eq!(world_position.y, 42); // 429/10
        assert_eq!(new_world_position.x, 11); // 117/10
        assert_eq!(new_world_position.y, 42); // 429/10
        assert_eq!(snapped_position.x, 110.0);
        assert_eq!(snapped_position.y, 420.0);
    }
}
*/
