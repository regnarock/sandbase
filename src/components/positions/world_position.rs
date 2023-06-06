use crate::components::positions::snapped_position::SnappedPosition;
use bevy::prelude::*;
use std::ops::{Add, Sub};

#[derive(Default, Component, Copy, Clone, Debug)]
pub struct WorldPosition {
    pub x: usize,
    pub y: usize,
}

impl WorldPosition {
    pub fn to_snapped(&self, px_per_voxel: usize) -> SnappedPosition {
        SnappedPosition::from_world_position(self, px_per_voxel)
    }
}

impl Sub for WorldPosition {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Add for WorldPosition {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
