use bevy::prelude::*;

use crate::components::positions::world_position::WorldPosition;
use crate::resources::world::config::WorldConfig;
use crate::resources::world::map::GameMap;

#[derive(Component, Copy, Clone, Debug, PartialEq)]
pub enum Voxel {
    OOB,
    of { data: VoxelStruct },
}

impl Voxel {
    pub fn update(
        &self,
        world_config: &WorldConfig,
        world: &mut GameMap,
        world_position: WorldPosition,
    ) -> Option<Move> {
        match self {
            Voxel::of {
                data: VoxelStruct { element: e, .. },
                ..
            } => match e {
                Element::Water => Voxel::update_water(world_config, world, world_position),
                Element::Sand => match Voxel::update_sand(world_config, world, world_position) {
                    Some(new_move) => Some(new_move),
                    _ => None,
                },
                Element::Earth => Voxel::update_earth(world_config, world, world_position),
            },
            // no-op for Out Of Bounds voxels
            Voxel::OOB => None,
        }
    }

    fn update_water(
        world_config: &WorldConfig,
        world: &mut GameMap,
        world_position: WorldPosition,
    ) -> Option<Move> {
        vec![
            world.get_bottom_voxel(world_position, world_config),
            world.get_bottom_left_voxel(world_position, world_config),
            world.get_bottom_right_voxel(world_position, world_config),
            world.get_left_voxel(world_position, world_config),
            world.get_right_voxel(world_position, world_config),
        ]
        .iter()
        .map(|(maybe_voxel, new_world_position)| {
            Voxel::liquid_behaviour(maybe_voxel, *new_world_position)
        })
        .find_map(|opt| opt)
    }

    fn update_sand(
        world_config: &WorldConfig,
        map: &mut GameMap,
        world_position: WorldPosition,
    ) -> Option<Move> {
        vec![
            map.get_bottom_voxel(world_position, world_config),
            map.get_bottom_left_voxel(world_position, world_config),
            map.get_bottom_right_voxel(world_position, world_config),
        ]
        .iter()
        .map(|(maybe_voxel, new_world_position)| {
            Voxel::falling_solid_behaviour(maybe_voxel, *new_world_position)
        })
        .find_map(|opt| opt)
    }

    fn update_earth(
        world_config: &WorldConfig,
        world: &mut GameMap,
        world_position: WorldPosition,
    ) -> Option<Move> {
        vec![
            world.get_bottom_voxel(world_position, world_config),
            world.get_bottom2_left_voxel(world_position, world_config),
            world.get_bottom2_right_voxel(world_position, world_config),
        ]
        .iter()
        .map(|(maybe_voxel, new_world_position)| {
            Voxel::falling_solid_behaviour(maybe_voxel, *new_world_position)
        })
        .find_map(|opt| opt)
    }

    fn falling_solid_behaviour(
        other_voxel: &Option<Voxel>,
        new_pos: WorldPosition,
    ) -> Option<Move> {
        match other_voxel {
            None => Some(Move::Displace(new_pos)),
            Some(voxel) => match voxel {
                Voxel::of {
                    data:
                        VoxelStruct {
                            kind: Kind::Liquid, ..
                        },
                    ..
                } => Some(Move::Swap(new_pos)),
                _ => None,
            },
        }
    }

    fn liquid_behaviour(other_voxel: &Option<Voxel>, new_pos: WorldPosition) -> Option<Move> {
        match other_voxel {
            None => Some(Move::Displace(new_pos)),
            Some(voxel) => match voxel {
                _ => None,
            },
        }
    }
}

#[derive(Component, Copy, Clone, Debug, PartialEq)]
pub struct VoxelStruct {
    pub size: usize,
    pub speed: f32,
    pub element: Element,
    pub kind: Kind,
    pub entity: Entity,
}

#[derive(Default, Resource)]
pub struct VoxelManager {
    pub sand_material: Handle<ColorMaterial>,
    pub water_material: Handle<ColorMaterial>,
    pub earth_material: Handle<ColorMaterial>,
}

impl VoxelManager {
    pub fn spawn_voxel(
        &self,
        world_config: &WorldConfig,
        element: Element,
        entity: Entity,
    ) -> VoxelStruct {
        match element {
            Element::Sand => VoxelStruct {
                size: world_config.px_per_voxel,
                speed: world_config.px_per_voxel as f32,
                element,
                kind: Kind::Solid,
                entity,
            },
            Element::Water => VoxelStruct {
                size: world_config.px_per_voxel,
                speed: world_config.px_per_voxel as f32,
                element,
                kind: Kind::Liquid,
                entity,
            },
            Element::Earth => VoxelStruct {
                size: world_config.px_per_voxel,
                speed: world_config.px_per_voxel as f32,
                element,
                kind: Kind::Solid,
                entity,
            },
        }
    }

    pub fn get_material(&self, element: Element) -> Handle<ColorMaterial> {
        match element {
            Element::Sand => self.sand_material.clone(),
            Element::Water => self.water_material.clone(),
            Element::Earth => self.earth_material.clone(),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Kind {
    Solid,
    Liquid,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Element {
    Sand,
    Water,
    Earth,
}

#[derive(Debug)]
pub enum Move {
    Displace(WorldPosition),
    Swap(WorldPosition),
}
