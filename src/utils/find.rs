use js_sys::Array;

use screeps_arena::{game::utils::find_in_range, GameObject};
use wasm_bindgen::JsCast;

pub fn find_closest_by_range(
    game_object: &GameObject,
    targets: &Option<Vec<GameObject>>,
) -> Option<GameObject> {
    if targets.is_some() {
        let targets = targets.as_ref().unwrap();
        let targets_array = Array::new();
        for target in targets {
            targets_array.push(target.as_ref());
        }
        let game_object_targets_closest = game_object.find_closest_by_range(targets_array.as_ref());
        if game_object_targets_closest.is_some() {
            Some(game_object_targets_closest.unwrap().unchecked_into())
        } else {
            None
        }
    } else {
        None
    }
}

pub fn find_in_x_range(
    game_object: &GameObject,
    targets: &Option<Vec<GameObject>>,
    range: u8,
) -> Option<Array> {
    if targets.is_some() {
        let targets = targets.as_ref().unwrap();
        let targets_array = Array::new();
        for target in targets {
            targets_array.push(target.as_ref());
        }
        let game_object_targets_array_in_range =
            find_in_range(game_object.unchecked_ref(), &targets_array, range);
        if game_object_targets_array_in_range.length() > 0 {
            Some(game_object_targets_array_in_range)
        } else {
            None
        }
    } else {
        None
    }
}
