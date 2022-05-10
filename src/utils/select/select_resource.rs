use screeps_arena::{
    constants::prototypes, game::utils::get_objects_by_prototype, GameObject, ResourceType::Energy,
};

pub fn select_energy() -> Option<Vec<GameObject>> {
    let resources = get_objects_by_prototype(prototypes::RESOURCE);
    if !resources.is_empty() {
        let mut resources_energy: Vec<GameObject> = Vec::new();
        for resource in resources {
            if resource.resource_type() == Energy {
                resources_energy.push(resource.into());
            }
        }
        if !resources_energy.is_empty() {
            Some(resources_energy)
        } else {
            None
        }
    } else {
        None
    }
}
