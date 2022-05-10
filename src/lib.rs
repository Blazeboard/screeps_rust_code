use log::*;
use screeps_arena::{
    constants::{prototypes, Part},
    game,
    prelude::*,
    ResourceType,
};
use wasm_bindgen::prelude::*;

use logic1::*;

mod logging;
mod logic1;
mod utils;

use crate::logic1::*;

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
        let (is_close, wall_line) = spawn::spawn();
        carry::carry();
        drop::drop();
        work::work();
        fight::fight((is_close, wall_line));
    }
}
