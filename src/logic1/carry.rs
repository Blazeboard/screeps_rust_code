use js_sys::Array;
use log::warn;
use screeps_arena::{
    constants::prototypes,
    game::utils::{self, get_objects_by_prototype},
    prelude::*,
    Creep, Resource, ResourceType, ReturnCode, StructureContainer, StructureExtension, Part,
};

// extension 和 creep 可能不存在，需要判断
pub fn carry() {
    let creeps = get_objects_by_prototype(prototypes::CREEP);
    let mut carriers = Vec::new();
    for creep in &creeps {
        let mut part_carry_num = 0;
        let mut part_move_num = 0;
        for body_part in creep.body() {
            if body_part.part() == Part::Carry {
                part_carry_num += 1;
            } else if body_part.part() == Part::Move {
                part_move_num += 1;
            }
            if part_carry_num == 2 && part_move_num == 2 {
                carriers.push(creep);
            }
        }
    }
    // warn!("{}",carriers.len());
    if !carriers.is_empty() {
        let spawns = get_objects_by_prototype(prototypes::STRUCTURE_SPAWN);
        let mut my_spawn: Option<screeps_arena::StructureSpawn> = None;
        for spawn in spawns {
            if spawn.my().unwrap_or(false) {
                my_spawn = Some(spawn);
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
        let mut extension_is_null: bool = false;
        if extensions.is_empty() {
            extension_is_null = true;
        }
        let mut my_free_extension: Vec<StructureExtension> = Vec::new();
        let my_free_extension_array = Array::new();
        if !extension_is_null {
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

            // 将Vec转换成Array，Array中的元素必须为JsValue，而StructureExtension类型即是
            for num in 0..my_free_extension.len() {
                my_free_extension_array.push(&my_free_extension[num]);
            }
        }

        // 将Vec转换成Array，Array中的元素必须为JsValue，而StructureContainer类型即是
        let full_container_array = Array::new();
        for num in 0..full_container.len() {
            full_container_array.push(&full_container[num]);
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
                            carrier.find_closest_by_range(&full_container_array);

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
                        if !extension_is_null {
                            let carrier_extension_closest =
                                carrier.find_closest_by_range(&my_free_extension_array);

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
                        }
                        carrier.move_to(my_spawn.clone().unwrap().as_ref(), None);
                        let capacity: u32 = carrier
                            .store()
                            .get_used_capacity(Some(ResourceType::Energy))
                            as u32;
                        carrier.transfer(
                            &my_spawn.clone().unwrap(),
                            ResourceType::Energy,
                            Some(capacity),
                        );
                    }
                }
            }
        } else if utils::get_ticks() > 240 {
            if carriers.len() > 0 {
                for carrier in carriers {
                    if carrier
                        .store()
                        .get_free_capacity(Some(ResourceType::Energy))
                        > 0
                    {
                        let carrier_container_closest =
                            carrier.find_closest_by_range(&full_container_array);
                        let resources = get_objects_by_prototype(prototypes::RESOURCE);
                        let mut resource_energy: Vec<Resource> = Vec::new();
                        for resource in resources {
                            if resource.resource_type() == ResourceType::Energy {
                                resource_energy.push(resource);
                            }
                        }
                        let resource_energy_array = Array::new();
                        for num in 0..resource_energy.len() {
                            resource_energy_array.push(resource_energy[num].as_ref());
                        }
                        let carrier_resource_closest =
                            carrier.find_closest_by_range(&resource_energy_array);
                        carrier.move_to(carrier_resource_closest.unwrap().as_ref(), None);
                        for resource in resource_energy {
                            if carrier.pickup(&resource) == ReturnCode::NotInRange {
                                continue;
                            }
                        }
                        carrier.move_to(carrier_container_closest.unwrap().as_ref(), None);
                        let capacity: u32 = carrier
                            .store()
                            .get_free_capacity(Some(ResourceType::Energy))
                            as u32;
                        for num in 0..full_container.len() {
                            if carrier.withdraw(
                                &full_container[num],
                                ResourceType::Energy,
                                Some(capacity),
                            ) == ReturnCode::NotInRange
                            {
                                continue;
                            }
                        }
                    } else {
                        if !extension_is_null {
                            let carrier_extension_closest =
                                carrier.find_closest_by_range(&my_free_extension_array);
                            carrier.move_to(carrier_extension_closest.unwrap().as_ref(), None);
                            let capacity: u32 = carrier
                                .store()
                                .get_used_capacity(Some(ResourceType::Energy))
                                as u32;
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
                        }
                        carrier.move_to(my_spawn.clone().unwrap().as_ref(), None);
                        let capacity: u32 = carrier
                            .store()
                            .get_used_capacity(Some(ResourceType::Energy))
                            as u32;
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
}
