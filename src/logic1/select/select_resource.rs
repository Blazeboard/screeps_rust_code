use screeps_arena::{
    constants::prototypes, game::utils::get_objects_by_prototype, Resource, ResourceType::Energy,
};

pub fn select_energy() -> Option<Vec<Resource>> {
    let resources = get_objects_by_prototype(prototypes::RESOURCE);
    if !resources.is_empty() {
        let mut resources_energy: Vec<Resource> = Vec::new();
        for resource in resources {
            if resource.resource_type() == Energy {
                resources_energy.push(resource);
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
