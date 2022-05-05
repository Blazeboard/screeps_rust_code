use js_sys::Array;
use log::warn;
use screeps_arena::{
    constants::prototypes,
    game::utils::{self, get_objects_by_prototype},
    prelude::*, Resource, ResourceType, ReturnCode,
};

use super::select::{select_creeps::select_droppers, select_structure::{select_my_spawn, select_out_containers}};

pub fn drop() {
    let droppers = select_droppers();
    if droppers.is_some() {
        let droppers = droppers.unwrap();
        let my_spawn = select_my_spawn();
        if droppers.len() > 0 {
            for droper in droppers {
                if droper.store().get_free_capacity(Some(ResourceType::Energy)) > 0 {
                    let out_containers = select_out_containers();
                }
            }
        }
    }
}