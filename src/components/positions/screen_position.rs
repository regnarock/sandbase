use bevy::prelude::*;
use std::ops::Sub;

use crate::components::positions::snapped_position::SnappedPosition;
use crate::resources::window::size::ScreenSize;
use crate::resources::world::config::WorldConfig;

#[derive(Default, Copy, Clone, Debug, PartialEq)]
pub struct ScreenPosition {
    pub x: f32,
    pub y: f32,
}

impl ScreenPosition {
    pub fn from_vec2(v: Vec2) -> Self {
        ScreenPosition { x: v.x, y: v.y }
    }

    pub fn from_vec3(v: Vec3) -> Self {
        ScreenPosition { x: v.x, y: v.y }
    }

    pub fn to_snapped(&self, world_config: &WorldConfig) -> SnappedPosition {
        SnappedPosition::from_screen_position(self, world_config)
    }

    pub fn to_vec3(&self) -> Vec3 {
        Vec3::new(self.x, self.y, 0.0)
    }
}

impl Sub for ScreenPosition {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
