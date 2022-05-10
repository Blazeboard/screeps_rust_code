use screeps_arena::{
    game::utils::{self},
    ResourceType::Energy,
};

use crate::utils::{
    find::find_closest_by_range,
    move_to_do::{move_to_pickup, move_to_transfer, move_to_withdraw, Action},
    select::{select_creeps::select_carriers, select_resource, select_structure},
};

pub fn carry() {
    let carriers = select_carriers();
    if carriers.is_some() {
        let carriers = carriers;
        let my_spawn = select_structure::select_my_spawn();
        let full_containers = select_structure::select_full_containers();
        let my_free_extensions = select_structure::select_free_extensions();

        if utils::get_ticks() <= 240 {
            if carriers.is_some() {
                let carriers = carriers.unwrap();
                for carrier in carriers {
                    if carrier.store().get_free_capacity(Some(Energy)) > 0 {
                        let carrier_container_closest =
                            find_closest_by_range(carrier.as_ref(), &full_containers);
                        move_to_withdraw(&carrier, &carrier_container_closest);
                    } else {
                        let carrier_extension_closest =
                            find_closest_by_range(carrier.as_ref(), &my_free_extensions);
                        if move_to_transfer(&carrier, &carrier_extension_closest, "extension")
                            .unwrap_or(Action::Did)
                            == Action::Did
                        {
                            move_to_transfer(&carrier, &my_spawn, "spawn");
                        }
                    }
                }
            }
        } else if utils::get_ticks() > 240 {
            if carriers.is_some() {
                let carriers = carriers.unwrap();
                for carrier in carriers {
                    if carrier.store().get_free_capacity(Some(Energy)) > 0 {
                        let resources_energy = select_resource::select_energy();

                        let carrier_resource_closest =
                            find_closest_by_range(carrier.as_ref(), &resources_energy);

                        move_to_pickup(&carrier, &carrier_resource_closest);

                        let carrier_container_closest =
                            find_closest_by_range(carrier.as_ref(), &full_containers);
                        move_to_withdraw(&carrier, &carrier_container_closest);
                    } else {
                        let carrier_extension_closest =
                            find_closest_by_range(carrier.as_ref(), &my_free_extensions);
                        if move_to_transfer(&carrier, &carrier_extension_closest, "extension")
                            .unwrap_or(Action::Did)
                            == Action::Did
                        {
                            move_to_transfer(&carrier, &my_spawn, "spawn");
                        }
                    }
                }
            }
        }
    }
}
