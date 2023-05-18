use bevy::{
    prelude::*,
};
use super::screen_position::*;
use super::world_position::*;
use crate::WorldConfig;

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

    pub fn to_world_position(&self, voxel_size: f32) -> WorldPosition {
        WorldPosition {
            x: (self.x / voxel_size) as usize,
            y: (self.y / voxel_size) as usize,
        }
    }

    pub fn from_world_position(world_pos: &WorldPosition, voxel_size: f32) -> SnappedPosition {
        SnappedPosition {
            x: world_pos.x as f32 * voxel_size,
            y: world_pos.y as f32 * voxel_size,
        }
    }

    pub fn from_screen_position(screen_pos: &ScreenPosition, world_config: &WorldConfig) -> SnappedPosition {
        let snapped_x = (screen_pos.x / world_config.voxel_size) * world_config.voxel_size;
        let snapped_y = (screen_pos.y / world_config.voxel_size) * world_config.voxel_size;
        SnappedPosition {
            x: snapped_x.min(world_config.pixels_width as f32 - 1.0).max(0.0),
            y: snapped_y.min(world_config.pixels_height as f32 - 1.0).max(0.0),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn screen_position_outside_world_gets_snapped_to_fit_in_world() {
        let world_config = WorldConfig::new(10, 144, 10);

        let screen_position = ScreenPosition {
            x: 640.0,
            y: 360.0,
        };

        let world_position = screen_position.to_snapped(world_config.voxel_size).to_world_position(world_config.voxel_size);
        let index = world_position.as_index(world_config.voxels_width);
        let snapped_position = world_position.to_snapped(world_config.voxel_size);

        assert_eq!(world_position.x, 64); // 640/10
        assert_eq!(world_position.y, 36); // 360/10
        assert_eq!(index, 9280); // 256*36+64
        assert_eq!(snapped_position.x, 640.0);
        assert_eq!(snapped_position.y, 360.0);
    }
}