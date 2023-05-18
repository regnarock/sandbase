use bevy::{
    prelude::*,
};
use crate::snapped_position::SnappedPosition;
use crate::WorldConfig;

#[derive(Default, Resource, Copy, Clone, Debug)]
pub struct ScreenPosition {
    pub x: f32,
    pub y: f32,
}

impl ScreenPosition {
    pub fn from_vec2(v: Vec2) -> Self {
        ScreenPosition {
            x: v.x,
            y: v.y,
        }
    }

    pub fn to_snapped(
        &self,
        world_config: &WorldConfig,
    ) -> SnappedPosition {
        SnappedPosition::from_screen_position(self, world_config)
    }

    pub fn to_vec3(&self) -> Vec3 {
        Vec3::new(self.x, self.y, 0.0)
    }
}