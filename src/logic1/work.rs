use js_sys::Array;

use screeps_arena::{
    game::utils::{self},
    prelude::*,
    ResourceType::{self, Energy},
    ReturnCode,
};
use wasm_bindgen::JsCast;

use super::select::{select_creeps, select_resource, select_structure};

pub fn work() {
    let my_spawn = select_structure::select_my_spawn();
    let workers = select_creeps::select_workers();
    let full_containers = select_structure::select_full_containers();
    let my_construction_sites = select_structure::select_my_construction_site();

    let full_containers_array = Array::new();
    if full_containers.is_some() {
        let full_containers = full_containers.unwrap();
        for full_container in full_containers {
            full_containers_array.push(full_container.as_ref());
        }
    }

    if workers.is_some() {
        let workers = workers.unwrap();
        for worker in workers {
            if utils::get_ticks() <= 300 {
                if my_construction_sites.is_some() {
                    let my_construction_sites = my_construction_sites.as_ref().unwrap();
                    // 建造逻辑
                    if !worker.store().get_used_capacity(Some(ResourceType::Energy)) > 0 {
                        let containers_in_home = select_structure::select_containers_in_home();
                        if !containers_in_home.is_some() {
                            let containers_in_home = containers_in_home.unwrap();
                            let containers_in_home_array = Array::new();
                            for container_in_home in containers_in_home {
                                containers_in_home_array.push(container_in_home.as_ref());
                            }
                            let worker_container_in_home_closest =
                                worker.find_closest_by_range(containers_in_home_array.as_ref());
                            let capacity = worker.store().get_free_capacity(Some(Energy)) as u32;
                            if worker.withdraw(
                                worker_container_in_home_closest
                                    .as_ref()
                                    .unwrap()
                                    .unchecked_ref(),
                                Energy,
                                Some(capacity),
                            ) == ReturnCode::NotInRange
                            {
                                worker.move_to(
                                    worker_container_in_home_closest.as_ref().unwrap(),
                                    None,
                                );
                            }
                        }
                    } else {
                        let my_construction_sites_array = Array::new();
                        for my_construction_site in my_construction_sites {
                            my_construction_sites_array.push(my_construction_site);
                        }
                        let worker_my_construction_sites_closest =
                            worker.find_closest_by_range(my_construction_sites_array.as_ref());
                        if worker.build(
                            worker_my_construction_sites_closest
                                .as_ref()
                                .unwrap()
                                .unchecked_ref(),
                        ) == ReturnCode::NotInRange
                        {
                            worker.move_to(
                                worker_my_construction_sites_closest.as_ref().unwrap(),
                                None,
                            );
                        }
                    }
                } else {
                    if worker.store().get_free_capacity(Some(Energy)) > 0 {
                        let my_spawn_full_containers_closest = my_spawn
                            .as_ref()
                            .unwrap()
                            .find_closest_by_range(full_containers_array.as_ref());
                        if worker.withdraw(
                            my_spawn_full_containers_closest
                                .as_ref()
                                .unwrap()
                                .unchecked_ref(),
                            Energy,
                            Some(worker.store().get_free_capacity(Some(Energy)) as u32),
                        ) == ReturnCode::NotInRange
                        {
                            worker
                                .move_to(my_spawn_full_containers_closest.as_ref().unwrap(), None);
                        }
                    } else {
                        if worker.transfer(
                            my_spawn.as_ref().unwrap(),
                            Energy,
                            Some(worker.store().get_free_capacity(Some(Energy)) as u32),
                        ) == ReturnCode::NotInRange
                        {
                            worker.move_to(my_spawn.as_ref().unwrap(), None);
                        }
                    }
                }
            } else {
                if my_construction_sites.is_some() {
                    let my_construction_sites = my_construction_sites.as_ref().unwrap();
                    // 建造逻辑
                    if !worker.store().get_used_capacity(Some(Energy)) > 0 {
                        let worker_full_container_closest =
                            worker.find_closest_by_range(full_containers_array.as_ref());

                        let resource_energy = select_resource::select_energy();

                        let capacity = worker.store().get_free_capacity(Some(Energy)) as u32;
                        if resource_energy.is_some() {
                            let resource_energy = resource_energy.unwrap();
                            let resource_energy_array = Array::new();
                            for energy in resource_energy {
                                resource_energy_array.push(energy.as_ref());
                            }
                            let worker_resource_energy_closest =
                                worker.find_closest_by_range(resource_energy_array.as_ref());
                            if worker.pickup(
                                worker_resource_energy_closest
                                    .as_ref()
                                    .unwrap()
                                    .unchecked_ref(),
                            ) == ReturnCode::NotInRange
                            {
                                worker.move_to(
                                    worker_resource_energy_closest.as_ref().unwrap(),
                                    None,
                                );
                            } else if worker.withdraw(
                                worker_full_container_closest
                                    .as_ref()
                                    .unwrap()
                                    .unchecked_ref(),
                                Energy,
                                Some(capacity),
                            ) == ReturnCode::NotInRange
                            {
                                worker.move_to(
                                    worker_full_container_closest
                                        .as_ref()
                                        .unwrap()
                                        .unchecked_ref(),
                                    None,
                                );
                            }
                        } else if worker.withdraw(
                            worker_full_container_closest
                                .as_ref()
                                .unwrap()
                                .unchecked_ref(),
                            Energy,
                            Some(capacity),
                        ) == ReturnCode::NotInRange
                        {
                            worker.move_to(
                                worker_full_container_closest
                                    .as_ref()
                                    .unwrap()
                                    .unchecked_ref(),
                                None,
                            );
                        }
                    } else {
                        let my_construction_sites_array = Array::new();
                        for my_construction_site in my_construction_sites {
                            my_construction_sites_array.push(my_construction_site);
                        }
                        let worker_my_construction_sites_closest =
                            worker.find_closest_by_range(my_construction_sites_array.as_ref());
                        if worker.build(
                            worker_my_construction_sites_closest
                                .as_ref()
                                .unwrap()
                                .unchecked_ref(),
                        ) == ReturnCode::NotInRange
                        {
                            worker.move_to(
                                worker_my_construction_sites_closest.as_ref().unwrap(),
                                None,
                            );
                        }
                    }
                } else {
                    if worker.store().get_free_capacity(Some(Energy)) > 0 {
                        let my_spawn_full_containers_closest = my_spawn
                            .as_ref()
                            .unwrap()
                            .find_closest_by_range(full_containers_array.as_ref());
                        if worker.withdraw(
                            my_spawn_full_containers_closest
                                .as_ref()
                                .unwrap()
                                .unchecked_ref(),
                            Energy,
                            Some(worker.store().get_free_capacity(Some(Energy)) as u32),
                        ) == ReturnCode::NotInRange
                        {
                            worker
                                .move_to(my_spawn_full_containers_closest.as_ref().unwrap(), None);
                        }
                    } else {
                        if worker.transfer(
                            my_spawn.as_ref().unwrap(),
                            Energy,
                            Some(worker.store().get_free_capacity(Some(Energy)) as u32),
                        ) == ReturnCode::NotInRange
                        {
                            worker.move_to(my_spawn.as_ref().unwrap(), None);
                        }
                    }
                }
            }
        }
    }
}
