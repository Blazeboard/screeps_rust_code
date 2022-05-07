use screeps_arena::ResourceType::Energy;

use crate::utils::{
    find::find_closest_by_range,
    move_to_do::move_to_withdraw,
    select::{select_creeps::select_droppers, select_structure::select_out_containers},
};

pub fn drop() {
    let droppers = select_droppers();
    if droppers.is_some() {
        let droppers = droppers.unwrap();
        if droppers.len() > 0 {
            for dropper in droppers {
                if dropper.store().get_free_capacity(Some(Energy)) > 0 {
                    let out_containers = select_out_containers();
                    let dropper_out_containers_closest =
                        find_closest_by_range(dropper.as_ref(), &out_containers);
                    move_to_withdraw(&dropper, &dropper_out_containers_closest);
                } else {
                    let capacity = dropper.store().get_capacity(Some(Energy));
                    dropper.drop(Energy, Some(capacity));
                }
            }
        }
    }
}
