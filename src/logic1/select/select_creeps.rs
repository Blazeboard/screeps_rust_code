use js_sys::Array;
use log::warn;
use screeps_arena::{
    constants::prototypes,
    game::utils::{self, get_objects_by_prototype},
    prelude::*,
    Creep, Part, Resource, ResourceType, ReturnCode, StructureContainer, StructureExtension,
    StructureSpawn,
};

pub fn select_carriers() -> Option<Vec<Creep>> {
    let creeps = get_objects_by_prototype(prototypes::CREEP);
    if !creeps.is_empty() {
        let mut carriers = Vec::new();
        for creep in creeps {
            let mut part_carry_num = 0;
            let mut part_move_num = 0;
            for body_part in creep.body() {
                if body_part.part() == Part::Carry {
                    part_carry_num += 1;
                } else if body_part.part() == Part::Move {
                    part_move_num += 1;
                }
                if part_carry_num == 2 && part_move_num == 2 {
                    carriers.push(creep.clone());
                }
            }
        }
        if !carriers.is_empty() {
            Some(carriers)
        } else {
            None
        }
    } else {
        None
    }
}

pub fn select_droppers() -> Option<Vec<Creep>> {
    let creeps = get_objects_by_prototype(prototypes::CREEP);
    if !creeps.is_empty() {
        let mut droppers = Vec::new();
        for creep in creeps {
            let mut part_carry_num = 0;
            let mut part_move_num = 0;
            for body_part in creep.body() {
                if body_part.part() == Part::Carry {
                    part_carry_num += 1;
                } else if body_part.part() == Part::Move {
                    part_move_num += 1;
                }
                if part_carry_num == 3 && part_move_num == 1 {
                    droppers.push(creep.clone());
                }
            }
        }
        if !droppers.is_empty() {
            Some(droppers)
        } else {
            None
        }
    } else {
        None
    }
}