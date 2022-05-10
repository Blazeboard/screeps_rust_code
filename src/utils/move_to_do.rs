use screeps_arena::{Creep, ResourceType::Energy, ReturnCode};
use screeps_arena::{
    GameObject, StructureContainer, StructureExtension, StructureSpawn, StructureTower,
};
use wasm_bindgen::JsCast;

#[derive(PartialEq)]
pub enum Action {
    Did,
    Moving,
}

pub fn move_to_withdraw(creep: &Creep, target: &Option<GameObject>) -> Option<Action> {
    if target.is_some() {
        let capacity: u32 = creep.store().get_free_capacity(Some(Energy)) as u32;
        if creep.withdraw(target.as_ref().unwrap(), Energy, Some(capacity))
            == ReturnCode::NotInRange
        {
            creep.move_to(target.as_ref().unwrap(), None);
            Some(Action::Moving)
        } else {
            Some(Action::Did)
        }
    } else {
        None
    }
}

pub fn move_to_transfer(
    creep: &Creep,
    target: &Option<GameObject>,
    target_type: &str,
) -> Option<Action> {
    if target.is_some() {
        let target = target.as_ref().unwrap();
        let capacity: u32 = creep.store().get_used_capacity(Some(Energy)) as u32;
        match target_type {
            "creep" => {
                if creep.transfer(target.unchecked_ref::<Creep>(), Energy, Some(capacity))
                    == ReturnCode::NotInRange
                {
                    creep.move_to(target.as_ref(), None);
                    Some(Action::Moving)
                } else {
                    Some(Action::Did)
                }
            }
            "container" => {
                if creep.transfer(
                    target.unchecked_ref::<StructureContainer>(),
                    Energy,
                    Some(capacity),
                ) == ReturnCode::NotInRange
                {
                    creep.move_to(target.as_ref(), None);
                    Some(Action::Moving)
                } else {
                    Some(Action::Did)
                }
            }
            "extension" => {
                if creep.transfer(
                    target.unchecked_ref::<StructureExtension>(),
                    Energy,
                    Some(capacity),
                ) == ReturnCode::NotInRange
                {
                    creep.move_to(target.as_ref(), None);
                    Some(Action::Moving)
                } else {
                    Some(Action::Did)
                }
            }
            "spawn" => {
                if creep.transfer(
                    target.unchecked_ref::<StructureSpawn>(),
                    Energy,
                    Some(capacity),
                ) == ReturnCode::NotInRange
                {
                    creep.move_to(target.as_ref(), None);
                    Some(Action::Moving)
                } else {
                    Some(Action::Did)
                }
            }
            "tower" => {
                if creep.transfer(
                    target.unchecked_ref::<StructureTower>(),
                    Energy,
                    Some(capacity),
                ) == ReturnCode::NotInRange
                {
                    creep.move_to(target.as_ref(), None);
                    Some(Action::Moving)
                } else {
                    Some(Action::Did)
                }
            }
            _ => None,
        }
    } else {
        None
    }
}

pub fn move_to_pickup(creep: &Creep, target: &Option<GameObject>) -> Option<Action> {
    if target.is_some() {
        if creep.pickup(target.as_ref().unwrap().unchecked_ref()) == ReturnCode::NotInRange {
            creep.move_to(target.as_ref().unwrap(), None);
            Some(Action::Moving)
        } else {
            Some(Action::Did)
        }
    } else {
        None
    }
}

pub fn move_to_build(creep: &Creep, target: &Option<GameObject>) -> Option<Action> {
    if target.is_some() {
        if creep.build(target.as_ref().unwrap().unchecked_ref()) == ReturnCode::NotInRange {
            creep.move_to(target.as_ref().unwrap(), None);
            Some(Action::Moving)
        } else {
            Some(Action::Did)
        }
    } else {
        None
    }
}
