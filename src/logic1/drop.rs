use js_sys::Array;
use log::warn;
use screeps_arena::{
    constants::prototypes,
    game::utils::{self, get_objects_by_prototype},
    prelude::*,
    Resource, ResourceType::Energy, ReturnCode,
};
use wasm_bindgen::JsCast;

use super::select::{
    select_creeps::select_droppers,
    select_structure::{select_my_spawn, select_out_containers},
};

pub fn drop() {
    let droppers = select_droppers();
    if droppers.is_some() {
        let droppers = droppers.unwrap();
        let my_spawn = select_my_spawn();
        if droppers.len() > 0 {
            for dropper in droppers {
                if dropper
                    .store()
                    .get_free_capacity(Some(Energy))
                    > 0
                {
                    let out_containers = select_out_containers();
                    if out_containers.is_some() {
                        let out_containers = out_containers.unwrap();
                        let out_containers_array = Array::new();
                        for out_container in out_containers {
                            out_containers_array.push(&out_container);
                        }
                        let dropper_out_containers_closest =
                            dropper.find_closest_by_range(&out_containers_array);
                        let capacity = dropper
                            .store()
                            .get_free_capacity(Some(Energy))
                            as u32;
                        if dropper.withdraw(
                            dropper_out_containers_closest
                                .as_ref()
                                .unwrap()
                                .unchecked_ref(),
                            Energy,
                            Some(capacity),
                        ) == ReturnCode::NotInRange
                        {
                            dropper.move_to(dropper_out_containers_closest.unwrap().as_ref(), None);
                        }
                    }
                } else {
                    let capacity = dropper.store().get_capacity(Some(Energy));
                    dropper.drop(Energy, Some(capacity));
                }
            }
        }
    }
}
