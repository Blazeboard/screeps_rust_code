use std::ops::Deref;

use js_sys::Array;
use log::warn;
use screeps_arena::{
    constants::prototypes,
    game::utils::{self, get_objects_by_prototype},
    prelude::*,
    Creep, Part, Resource, ResourceType, ReturnCode, StructureContainer, StructureExtension,
    StructureSpawn,
};

pub fn select_my_spawn() -> Option<StructureSpawn> {
    let spawns = get_objects_by_prototype(prototypes::STRUCTURE_SPAWN);
    let mut my_spawn: Option<screeps_arena::StructureSpawn> = None;
    for spawn in spawns {
        if spawn.my().unwrap_or(false) {
            my_spawn = Some(spawn);
        }
    }
    my_spawn
}

pub fn select_full_containers() -> Option<Vec<StructureContainer>> {
    let containers = get_objects_by_prototype(prototypes::STRUCTURE_CONTAINER);
    if !containers.is_empty() {
        let mut full_containers: Vec<StructureContainer> = Vec::new();
        for container in containers {
            if container
                .store()
                .get_used_capacity(Some(ResourceType::Energy))
                > 0
            {
                full_containers.push(container);
            }
        }
        if !full_containers.is_empty() {
            Some(full_containers)
        } else {
            None
        }
    } else {
        None
    }
}

pub fn select_free_extensions() -> Option<Vec<StructureExtension>> {
    let extensions = get_objects_by_prototype(prototypes::STRUCTURE_EXTENSION);
    if !extensions.is_empty() {
        let mut my_free_extensions: Vec<StructureExtension> = Vec::new();
        for extension in extensions {
            if extension.my().unwrap_or(false)
                && extension
                    .store()
                    .get_free_capacity(Some(ResourceType::Energy))
                    > 0
            {
                my_free_extensions.push(extension);
            }
        }
        if !my_free_extensions.is_empty() {
            Some(my_free_extensions)
        } else {
            None
        }
    } else {
        None
    }
}

pub fn select_out_containers() -> Option<Vec<StructureContainer>> {
    let my_spawn = select_my_spawn().unwrap();
    let containers = get_objects_by_prototype(prototypes::STRUCTURE_CONTAINER);
    if !containers.is_empty() {
        let mut out_containers: Vec<StructureContainer> = Vec::new();
        for container in containers {
            if container
                .store()
                .get_used_capacity(Some(ResourceType::Energy))
                > 0
                && match container.x().checked_sub(my_spawn.x()) {
                    Some(x) => x,
                    None => my_spawn.x() - container.x(),
                } > 7
            {
                out_containers.push(container);
            }
        }
        if !out_containers.is_empty() {
            Some(out_containers)
        } else {
            None
        }
    } else {
        None
    }
}
