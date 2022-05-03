use std::convert::TryInto;

use js_sys::Array;
use screeps_arena::{
    constants::{prototypes, Part},
    game::{
        self,
        utils::{self, get_objects_by_prototype},
    },
    prelude::*,
    Creep, ResourceType, ReturnCode, StructureContainer, StructureExtension,
};
use wasm_bindgen::JsValue;

fn carry(carriers: &Vec<Creep>) {
    let spawns = get_objects_by_prototype(prototypes::STRUCTURE_SPAWN);
    let mut my_spawn: Option<screeps_arena::StructureSpawn> = None;
    let mut enemy_spawn: Option<screeps_arena::StructureSpawn> = None;
    for spawn in spawns {
        if spawn.my().unwrap_or(false) {
            my_spawn = Some(spawn);
        } else {
            enemy_spawn = Some(spawn);
        }
    }
    let containers = get_objects_by_prototype(prototypes::STRUCTURE_CONTAINER);
    let mut full_container: Vec<StructureContainer> = Vec::new();
    for container in containers {
        if container
            .store()
            .get_used_capacity(Some(ResourceType::Energy))
            > 0
        {
            full_container.push(container);
        }
    }
    let extensions = get_objects_by_prototype(prototypes::STRUCTURE_EXTENSION);
    let mut my_free_extension: Vec<StructureExtension> = Vec::new();
    for extension in extensions {
        if extension.my().unwrap_or(false)
            && extension
                .store()
                .get_free_capacity(Some(ResourceType::Energy))
                > 0
        {
            my_free_extension.push(extension);
        }
    }

    // 将Vec转换成Array，Array中的元素必须为JsValue，而StructureContainer类型即是
    let mut full_container_Array = Array::new();
    for num in 0..full_container.len() {
        full_container_Array.push(&full_container[num]);
    }

    let mut my_free_extension_Array = Array::new();
    for num in 0..my_free_extension.len() {
        my_free_extension_Array.push(&my_free_extension[num]);
    }

    if utils::get_ticks() <= 240 {
        if carriers.len() > 0 {
            for carrier in carriers {
                if carrier
                    .store()
                    .get_free_capacity(Some(ResourceType::Energy))
                    > 0
                {
                    let carrier_container_closest =
                        carrier.find_closest_by_range(&full_container_Array);

                    carrier.move_to(carrier_container_closest.unwrap().as_ref(), None);
                    let capacity: u32 = carrier
                        .store()
                        .get_free_capacity(Some(ResourceType::Energy))
                        as u32;
                    for num in 0..full_container.len() {
                        if carrier.withdraw(
                            full_container[num].as_ref(),
                            ResourceType::Energy,
                            Some(capacity),
                        ) == ReturnCode::NotInRange
                        {
                            continue;
                        }
                    }
                } else {
                    let carrier_extension_closest =
                        carrier.find_closest_by_range(&my_free_extension_Array);

                    carrier.move_to(carrier_extension_closest.unwrap().as_ref(), None);
                    let capacity: u32 =
                        carrier.store().get_capacity(Some(ResourceType::Energy)) as u32;
                    for num in 0..my_free_extension.len() {
                        if carrier.transfer(
                            &my_free_extension[num],
                            ResourceType::Energy,
                            Some(capacity),
                        ) == ReturnCode::NotInRange
                        {
                            continue;
                        }
                    }
                    carrier.move_to(my_spawn.clone().unwrap().as_ref(), None);
                    let capacity: u32 =
                        carrier.store().get_capacity(Some(ResourceType::Energy)) as u32;
                    carrier.transfer(
                        &my_spawn.clone().unwrap(),
                        ResourceType::Energy,
                        Some(capacity),
                    );
                }
            }
        }
    }
}
