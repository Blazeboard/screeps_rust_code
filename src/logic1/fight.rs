use js_sys::Array;

use screeps_arena::{
    game::utils::{self},
    prelude::*,
    ResourceType::Energy,
    ReturnCode, StructureExtension,
};
use wasm_bindgen::JsCast;

use super::select::{select_creeps, select_structure};

pub fn fight() {
    let my_spawn = select_structure::select_my_spawn();
    let enemy_creeps = select_creeps::select_enemy_creeps();
    let enemy_spawn = select_structure::select_enemy_spawn();
    let enemy_extensions = select_structure::select_enemy_extensions();
    let enemy_tower = select_structure::select_enemy_tower();
    let mages = select_creeps::select_mages();
    let my_injured_creeps = select_creeps::select_my_injured_creeps();

}