use js_sys::Array;

use screeps_arena::GameObject;
use wasm_bindgen::JsCast;

pub fn find_closest_by_range(game_object: &GameObject, targets: &Option<Vec<GameObject>>) -> Option<GameObject> {
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

