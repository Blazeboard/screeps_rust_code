use screeps_arena::game::pathfinder::CostMatrix;
use screeps_arena::game::utils::{get_objects_by_prototype, get_range};
use screeps_arena::{prototypes, Creep, GameObject, OwnedStructureProperties};
use wasm_bindgen::JsCast;

use super::find;

pub fn set_obstacles(cost_matrix: CostMatrix) -> CostMatrix {
    let obstacle_spawns = get_objects_by_prototype(prototypes::STRUCTURE_SPAWN);
    for obstacle_spawn in obstacle_spawns {
        if obstacle_spawn.exists() {
            cost_matrix.set(obstacle_spawn.x(), obstacle_spawn.y(), 255);
        }
    }

    let obstacle_extensions = get_objects_by_prototype(prototypes::STRUCTURE_EXTENSION);
    for obstacle_extension in obstacle_extensions {
        if obstacle_extension.exists() {
            cost_matrix.set(obstacle_extension.x(), obstacle_extension.y(), 255);
        }
    }

    let obstacle_towers = get_objects_by_prototype(prototypes::STRUCTURE_TOWER);
    for obstacle_tower in obstacle_towers {
        if obstacle_tower.exists() {
            cost_matrix.set(obstacle_tower.x(), obstacle_tower.y(), 255);
        }
    }

    let obstacle_walls = get_objects_by_prototype(prototypes::STRUCTURE_WALL);
    for obstacle_wall in obstacle_walls {
        if obstacle_wall.exists() {
            cost_matrix.set(obstacle_wall.x(), obstacle_wall.y(), 255);
        }
    }

    let obstacle_ramparts = get_objects_by_prototype(prototypes::STRUCTURE_RAMPART);
    for obstacle_rampart in obstacle_ramparts {
        if obstacle_rampart.exists() && !obstacle_rampart.my().unwrap_or(false) {
            cost_matrix.set(obstacle_rampart.x(), obstacle_rampart.y(), 255);
        }
    }

    let obstacle_creeps = get_objects_by_prototype(prototypes::CREEP);
    for obstacle_creep in obstacle_creeps {
        if obstacle_creep.exists() && !obstacle_creep.my() {
            cost_matrix.set(obstacle_creep.x(), obstacle_creep.y(), 255);
            cost_matrix.set(obstacle_creep.x() + 1, obstacle_creep.y(), 255);
            cost_matrix.set(obstacle_creep.x() - 1, obstacle_creep.y(), 255);
            cost_matrix.set(obstacle_creep.x() + 1, obstacle_creep.y() + 1, 255);
            cost_matrix.set(obstacle_creep.x() - 1, obstacle_creep.y() + 1, 255);
            cost_matrix.set(obstacle_creep.x() + 1, obstacle_creep.y() - 1, 255);
            cost_matrix.set(obstacle_creep.x() - 1, obstacle_creep.y() - 1, 255);
            cost_matrix.set(obstacle_creep.x(), obstacle_creep.y() + 1, 255);
            cost_matrix.set(obstacle_creep.x(), obstacle_creep.y() - 1, 255);
        }
    }
    cost_matrix
}

pub fn is_near_to_teammates(creep: &Creep, teammates: &Option<Vec<GameObject>>) -> bool {
    let creep_teammates_closest = find::find_closest_by_range(creep, teammates);
    if creep_teammates_closest.is_some() {
        if get_range(
            creep.unchecked_ref(),
            creep_teammates_closest.unwrap().unchecked_ref(),
        ) == 1
        {
            true
        } else {
            false
        }
    } else {
        false
    }
}
