use log::warn;
use screeps_arena::{
    game::utils::{self},
    ResourceType::{self, Energy},
};

use crate::utils::{
    find::find_closest_by_range,
    move_to_do::{move_to_build, move_to_pickup, move_to_transfer, move_to_withdraw, Action},
    select::{select_creeps, select_resource, select_structure},
};

pub fn work() {
    let my_spawn = select_structure::select_my_spawn();
    let workers = select_creeps::select_workers();
    let full_containers = select_structure::select_full_containers();
    let my_construction_sites = select_structure::select_my_construction_sites();

    if workers.is_some() {
        let workers = workers.unwrap();
        for worker in workers {
            if utils::get_ticks() <= 300 {
                if my_construction_sites.is_some() {
                    // 建造逻辑
                    if !(worker.store().get_used_capacity(Some(ResourceType::Energy)) > 0) {
                        let containers_in_home = select_structure::select_containers_in_home();                        
                        let worker_container_in_home_closest =
                            find_closest_by_range(worker.as_ref(), &containers_in_home);
                        move_to_withdraw(&worker, &worker_container_in_home_closest);
                        warn!("***{}", worker.store().get_used_capacity(Some(ResourceType::Energy)));
                        warn!("***{}", worker.store().get(ResourceType::Energy).unwrap());
                    } else {
                        let worker_my_construction_sites_closest =
                            find_closest_by_range(worker.as_ref(), &my_construction_sites);
                        move_to_build(&worker, &worker_my_construction_sites_closest);
                    }
                } else {
                    if worker.store().get_free_capacity(Some(Energy)) > 0 {
                        let my_spawn_full_containers_closest =
                            find_closest_by_range(my_spawn.as_ref().unwrap(), &full_containers);
                        move_to_withdraw(&worker, &my_spawn_full_containers_closest);
                    } else {
                        move_to_transfer(&worker, &my_spawn, "spawn");
                    }
                }
            } else {
                if my_construction_sites.is_some() {
                    // 建造逻辑
                    if !(worker.store().get_used_capacity(Some(Energy)) > 0) {
                        let worker_full_container_closest =
                            find_closest_by_range(worker.as_ref(), &full_containers);
                        let resource_energy = select_resource::select_energy();
                        let worker_resource_energy_closest =
                            find_closest_by_range(worker.as_ref(), &resource_energy);
                        if move_to_pickup(&worker, &worker_resource_energy_closest)
                            .unwrap_or(Action::Did)
                            == Action::Did
                        {
                            move_to_withdraw(&worker, &worker_full_container_closest);
                        }
                    } else {
                        let worker_my_construction_sites_closest =
                            find_closest_by_range(worker.as_ref(), &my_construction_sites);
                        move_to_build(&worker, &worker_my_construction_sites_closest);
                    }
                } else {
                    if worker.store().get_free_capacity(Some(Energy)) > 0 {
                        let my_spawn_full_containers_closest =
                            find_closest_by_range(my_spawn.as_ref().unwrap(), &full_containers);
                        move_to_withdraw(&worker, &my_spawn_full_containers_closest);
                    } else {
                        move_to_transfer(&worker, &my_spawn, "spawn");
                    }
                }
            }
        }
    }
}
