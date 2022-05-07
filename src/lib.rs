use log::*;
use screeps_arena::{
    prelude::*,
    game,
    constants::{prototypes, Part}, ResourceType,
};
use wasm_bindgen::prelude::*;

use logic1::*;

mod logging;
mod logic1;
mod utils;

fn setup() {
    logging::setup_logging(logging::Info);
}

// add wasm_bindgen to any function you would like to expose for call from js
// to use a reserved name as a function name, use `js_name`:
#[wasm_bindgen(js_name = loop)]
pub fn tick() {
    let tick = game::utils::get_ticks();

    if tick == 1 {
        setup();
    }
    // warn!("hello arena! {}", tick);

    // let info = game::arena_info();
    // warn!("arena_info: {:?}", info);

    // strategy for spawn and swamp arena, which will conditionally compile in
    // only when this feature is enabled for the crate
    #[cfg(feature = "arena-spawn-and-swamp")]
    {
        let carrier_body = [Part::Move, Part::Carry, Part::Move, Part::Carry];
        let dropper_body = [Part::Carry, Part::Carry, Part::Carry, Part::Move];
        let mut enemy_spawn = None;
        let spawns = game::utils::get_objects_by_prototype(prototypes::STRUCTURE_SPAWN);
        // warn!("spawns {}", spawns.len());
        for spawn in spawns {
            if spawn.my().unwrap_or(false) {
                spawn.spawn_creep(&carrier_body);
            } else {
                enemy_spawn = Some(spawn);
            }
        }

        carry::carry();
        // drop::drop()
        
        // let mut carriers_not_none = Vec::new();
        // for carrier in &carriers {
        //     if carrier.is_some() {
        //         carriers_not_none.push(carrier.clone().unwrap());
        //     }
        // }
        // warn!("{}", carriers.is_empty());
        // warn!("{}", carriers_not_none.is_empty());
        // for carrier in carriers_not_none {
        //     let capacity = carrier.store().get_free_capacity(Some(ResourceType::Energy));
        //     warn!("{}", capacity);
        // }

        // carry::carry(&carriers);
        // let extensions = game::utils::get_objects_by_prototype(prototypes::STRUCTURE_EXTENSION);
        
        // warn!("{}", extensions.is_empty());
        // for container in &containers {
        //     // let capacity = container.store().get_used_capacity(Some(screeps_arena::ResourceType::Energy));
        //     if !container.exists() {
        //         warn!("{}", 1);
        //     }
        // }

        // let creeps = game::utils::get_objects_by_prototype(prototypes::CREEP);
        // // warn!("creeps {}", creeps.len());
        // for creep in creeps {
        //     if creep.my() {
        //         match &enemy_spawn {
        //             Some(t) => {
        //                 creep.move_to(t.as_ref(), None);
        //                 creep.attack(t);
        //             }
        //             None => {}
        //         }
        //     }
        // }
    }
    
}