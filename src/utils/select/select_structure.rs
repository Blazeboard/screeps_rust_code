use screeps_arena::{
    constants::prototypes,
    game::utils::{self, get_objects_by_prototype},
    prelude::*,
    GameObject,
    ResourceType::Energy,
};
use wasm_bindgen::JsCast;

pub fn select_my_spawn() -> Option<GameObject> {
    let spawns = get_objects_by_prototype(prototypes::STRUCTURE_SPAWN);
    let mut my_spawn: Option<GameObject> = None;
    for spawn in spawns {
        if spawn.my().unwrap_or(false) {
            my_spawn = Some(spawn.into());
        }
    }
    my_spawn
}

pub fn select_enemy_spawn() -> Option<GameObject> {
    let spawns = get_objects_by_prototype(prototypes::STRUCTURE_SPAWN);
    let mut enemy_spawn: Option<GameObject> = None;
    for spawn in spawns {
        if !spawn.my().unwrap_or(true) {
            enemy_spawn = Some(spawn.into());
        }
    }
    enemy_spawn
}

pub fn select_full_containers() -> Option<Vec<GameObject>> {
    let containers = get_objects_by_prototype(prototypes::STRUCTURE_CONTAINER);
    if !containers.is_empty() {
        let mut full_containers: Vec<GameObject> = Vec::new();
        for container in containers {
            if container.store().get_used_capacity(Some(Energy)) > 0 {
                full_containers.push(container.into());
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

pub fn select_free_extensions() -> Option<Vec<GameObject>> {
    let extensions = get_objects_by_prototype(prototypes::STRUCTURE_EXTENSION);
    if !extensions.is_empty() {
        let mut my_free_extensions: Vec<GameObject> = Vec::new();
        for extension in extensions {
            if extension.my().unwrap_or(false)
                && extension.store().get_free_capacity(Some(Energy)) > 0
            {
                my_free_extensions.push(extension.into());
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

pub fn select_out_containers() -> Option<Vec<GameObject>> {
    let my_spawn = select_my_spawn().unwrap();
    let enemy_spawn = select_enemy_spawn().unwrap();
    let containers = get_objects_by_prototype(prototypes::STRUCTURE_CONTAINER);
    if !containers.is_empty() {
        let mut out_containers: Vec<GameObject> = Vec::new();
        for container in containers {
            if container.store().get_used_capacity(Some(Energy)) > 0
                && utils::get_range(my_spawn.unchecked_ref(), container.unchecked_ref()) > 8
                && utils::get_range(enemy_spawn.unchecked_ref(), container.unchecked_ref()) > 8
            {
                out_containers.push(container.into());
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

pub fn select_containers_in_home() -> Option<Vec<GameObject>> {
    let my_spawn = select_my_spawn().unwrap();
    let containers = get_objects_by_prototype(prototypes::STRUCTURE_CONTAINER);
    if !containers.is_empty() {
        let mut containers_in_home: Vec<GameObject> = Vec::new();
        for container in containers {
            if container.store().get_used_capacity(Some(Energy)) > 0
                && utils::get_range(my_spawn.unchecked_ref(), container.unchecked_ref()) <= 8
            {
                containers_in_home.push(container.into());
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

pub fn select_my_construction_sites() -> Option<Vec<GameObject>> {
    let construction_sites = get_objects_by_prototype(prototypes::CONSTRUCTION_SITE);
    if !construction_sites.is_empty() {
        let mut my_construction_sites = Vec::new();
        for construction_site in construction_sites {
            if construction_site.my() {
                my_construction_sites.push(construction_site.into());
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

pub fn select_enemy_extensions() -> Option<Vec<GameObject>> {
    let extensions = get_objects_by_prototype(prototypes::STRUCTURE_EXTENSION);
    if !extensions.is_empty() {
        let mut enemy_extensions = Vec::new();
        for extension in extensions {
            if !extension.my().unwrap() {
                enemy_extensions.push(extension.into());
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

pub fn select_enemy_towers() -> Option<Vec<GameObject>> {
    let towers = get_objects_by_prototype(prototypes::STRUCTURE_TOWER);
    if !towers.is_empty() {
        let mut enemy_towers = Vec::new();
        for tower in towers {
            if !tower.my().unwrap_or(true) {
                enemy_towers.push(tower.into());
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
