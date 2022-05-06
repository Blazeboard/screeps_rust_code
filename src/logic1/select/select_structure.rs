use screeps_arena::{
    constants::prototypes,
    game::utils::{self, get_objects_by_prototype},
    prelude::*,
    ResourceType::Energy,
    StructureContainer, StructureExtension, StructureSpawn, StructureTower, ConstructionSite,
};
use wasm_bindgen::JsCast;

pub fn select_my_spawn() -> Option<StructureSpawn> {
    let spawns = get_objects_by_prototype(prototypes::STRUCTURE_SPAWN);
    let mut my_spawn: Option<StructureSpawn> = None;
    for spawn in spawns {
        if spawn.my().unwrap_or(false) {
            my_spawn = Some(spawn);
        }
    }
    my_spawn
}

pub fn select_enemy_spawn() -> Option<StructureSpawn> {
    let spawns = get_objects_by_prototype(prototypes::STRUCTURE_SPAWN);
    let mut enemy_spawn: Option<StructureSpawn> = None;
    for spawn in spawns {
        if !spawn.my().unwrap_or(true) {
            enemy_spawn = Some(spawn);
        }
    }
    enemy_spawn
}

pub fn select_full_containers() -> Option<Vec<StructureContainer>> {
    let containers = get_objects_by_prototype(prototypes::STRUCTURE_CONTAINER);
    if !containers.is_empty() {
        let mut full_containers: Vec<StructureContainer> = Vec::new();
        for container in containers {
            if container.store().get_used_capacity(Some(Energy)) > 0 {
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
                && extension.store().get_free_capacity(Some(Energy)) > 0
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
            if container.store().get_used_capacity(Some(Energy)) > 0
                && utils::get_range(my_spawn.unchecked_ref(), container.unchecked_ref()) > 8
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

pub fn select_containers_in_home() -> Option<Vec<StructureContainer>> {
    let my_spawn = select_my_spawn().unwrap();
    let containers = get_objects_by_prototype(prototypes::STRUCTURE_CONTAINER);
    if !containers.is_empty() {
        let mut containers_in_home: Vec<StructureContainer> = Vec::new();
        for container in containers {
            if container.store().get_used_capacity(Some(Energy)) > 0
                && utils::get_range(my_spawn.unchecked_ref(), container.unchecked_ref()) <= 8
            {
                containers_in_home.push(container);
            }
        }
        if !containers_in_home.is_empty() {
            Some(containers_in_home)
        } else {
            None
        }
    } else {
        None
    }
}

pub fn select_my_construction_site() -> Option<Vec<ConstructionSite>> {
    let construction_sites = get_objects_by_prototype(prototypes::CONSTRUCTION_SITE);
    if !construction_sites.is_empty() {
        let mut my_construction_sites = Vec::new();
        for construction_site in construction_sites {
            if construction_site.my() {
                my_construction_sites.push(construction_site);
            }
        }
        if !my_construction_sites.is_empty() {
            Some(my_construction_sites)
        } else {
            None
        }
    } else {
        None
    }
}

pub fn select_enemy_extensions() -> Option<Vec<StructureExtension>> {
    let extensions = get_objects_by_prototype(prototypes::STRUCTURE_EXTENSION);
    if !extensions.is_empty() {
        let mut enemy_extensions = Vec::new();
        for extension in extensions {
            if !extension.my().unwrap() {
                enemy_extensions.push(extension);
            }
        }
        if !enemy_extensions.is_empty() {
            Some(enemy_extensions)
        } else {
            None
        }
    } else {
        None
    }
}

pub fn select_enemy_tower() -> Option<Vec<StructureTower>> {
    let towers = get_objects_by_prototype(prototypes::STRUCTURE_TOWER);
    if !towers.is_empty() {
        let mut enemy_towers = Vec::new();
        for tower in towers {
            if !tower.my().unwrap_or(true) {
                enemy_towers.push(tower);
            }
        }
        if !enemy_towers.is_empty() {
            Some(enemy_towers)
        } else {
            None
        }
    } else {
        None
    }
}
