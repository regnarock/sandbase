use bevy::{
    prelude::*,
};
use crate::positions::snapped_position::SnappedPosition;

#[derive(Default, Resource, Copy, Clone, Debug)]
pub struct WorldPosition {
    pub x: usize,
    pub y: usize,
}

impl WorldPosition {
    pub fn to_snapped(
        &self,
        voxel_size: f32,
    ) -> SnappedPosition {
        SnappedPosition::from_world_position(self, voxel_size)
    }
}