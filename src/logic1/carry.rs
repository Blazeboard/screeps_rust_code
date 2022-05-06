use js_sys::Array;
use log::warn;
use screeps_arena::{
    constants::prototypes,
    game::utils::{self, get_objects_by_prototype},
    prelude::*, Resource, ResourceType::Energy, ReturnCode,
};

use super::select::{select_creeps::select_carriers, select_structure};

pub fn carry() {
    let carriers = select_carriers();
    if carriers.is_some() {
        let carriers = carriers.unwrap();
        let my_spawn = select_structure::select_my_spawn();
        let full_containers = select_structure::select_full_containers().unwrap();
        let my_free_extensions = select_structure::select_free_extensions();

        // 将Vec转换成Array，Array中的元素必须为JsValue，而StructureExtension类型即是
        let my_free_extensions_array = Array::new();
        if my_free_extensions.is_some() {
            let my_free_extensions = my_free_extensions.clone().unwrap();
            for num in 0..my_free_extensions.len() {
                my_free_extensions_array.push(&my_free_extensions[num]);
            }
        }

        // 将Vec转换成Array，Array中的元素必须为JsValue，而StructureContainer类型即是
        let full_container_array = Array::new();
        for num in 0..full_containers.len() {
            full_container_array.push(&full_containers[num]);
        }

        if utils::get_ticks() <= 240 {
            if carriers.len() > 0 {
                for carrier in carriers {
                    if carrier
                        .store()
                        .get_free_capacity(Some(Energy))
                        > 0
                    {
                        let carrier_container_closest =
                            carrier.find_closest_by_range(&full_container_array);

                        carrier.move_to(carrier_container_closest.unwrap().as_ref(), None);
                        let capacity: u32 = carrier
                            .store()
                            .get_free_capacity(Some(Energy))
                            as u32;
                        for num in 0..full_containers.len() {
                            if carrier.withdraw(
                                full_containers[num].as_ref(),
                                Energy,
                                Some(capacity),
                            ) == ReturnCode::NotInRange
                            {
                                continue;
                            }
                        }
                    } else {
                        if my_free_extensions.is_some() {
                            let my_free_extensions = my_free_extensions.clone().unwrap();
                            let carrier_extension_closest =
                                carrier.find_closest_by_range(&my_free_extensions_array);

                            carrier.move_to(carrier_extension_closest.unwrap().as_ref(), None);
                            let capacity: u32 =
                                carrier.store().get_capacity(Some(Energy)) as u32;
                            for num in 0..my_free_extensions.len() {
                                if carrier.transfer(
                                    &my_free_extensions[num],
                                    Energy,
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
                            .get_used_capacity(Some(Energy))
                            as u32;
                        carrier.transfer(
                            &my_spawn.clone().unwrap(),
                            Energy,
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
                        .get_free_capacity(Some(Energy))
                        > 0
                    {
                        let carrier_container_closest =
                            carrier.find_closest_by_range(&full_container_array);
                        let resources = get_objects_by_prototype(prototypes::RESOURCE);
                        let mut resource_energy: Vec<Resource> = Vec::new();
                        for resource in resources {
                            if resource.resource_type() == Energy {
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
                            .get_free_capacity(Some(Energy))
                            as u32;
                        for num in 0..full_containers.len() {
                            if carrier.withdraw(
                                &full_containers[num],
                                Energy,
                                Some(capacity),
                            ) == ReturnCode::NotInRange
                            {
                                continue;
                            }
                        }
                    } else {
                        if my_free_extensions.is_some() {
                            let my_free_extensions = my_free_extensions.clone().unwrap();
                            let carrier_extension_closest =
                                carrier.find_closest_by_range(&my_free_extensions_array);
                            carrier.move_to(carrier_extension_closest.unwrap().as_ref(), None);
                            let capacity: u32 = carrier
                                .store()
                                .get_used_capacity(Some(Energy))
                                as u32;
                            for num in 0..my_free_extensions.len() {
                                if carrier.transfer(
                                    &my_free_extensions[num],
                                    Energy,
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
                            .get_used_capacity(Some(Energy))
                            as u32;
                        carrier.transfer(
                            &my_spawn.clone().unwrap(),
                            Energy,
                            Some(capacity),
                        );
                    }
                }
            }
        }
    }
}
