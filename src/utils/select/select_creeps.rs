use screeps_arena::{constants::prototypes, game::utils::get_objects_by_prototype, Creep, Part, GameObject};

pub fn select_my_injured_creeps() -> Option<Vec<Creep>> {
    let creeps = get_objects_by_prototype(prototypes::CREEP);
    if !creeps.is_empty() {
        let mut my_injured_creeps = Vec::new();
        for creep in creeps {
            if creep.my() && creep.hits() < creep.hits_max() {
                my_injured_creeps.push(creep);
            }
        }
        if !my_injured_creeps.is_empty() {
            Some(my_injured_creeps)
        } else {
            None
        }
    } else {
        None
    }
}

pub fn select_carriers() -> Option<Vec<Creep>> {
    let creeps = get_objects_by_prototype(prototypes::CREEP);
    if !creeps.is_empty() {
        let mut carriers = Vec::new();
        for creep in creeps {
            let mut part_carry_num = 0;
            let mut part_move_num = 0;
            for body_part in creep.body() {
                match body_part.part() {
                    Part::Carry => part_carry_num += 1,
                    Part::Move => part_move_num += 1,
                    _ => (),
                }
            }
            if part_carry_num == 2 && part_move_num == 2 {
                carriers.push(creep);
            }
        }
        if !carriers.is_empty() {
            Some(carriers)
        } else {
            None
        }
    } else {
        None
    }
}

pub fn select_droppers() -> Option<Vec<Creep>> {
    let creeps = get_objects_by_prototype(prototypes::CREEP);
    if !creeps.is_empty() {
        let mut droppers = Vec::new();
        for creep in creeps {
            let mut part_carry_num = 0;
            let mut part_move_num = 0;
            for body_part in creep.body() {
                match body_part.part() {
                    Part::Carry => part_carry_num += 1,
                    Part::Move => part_move_num += 1,
                    _ => (),
                }
            }
            if part_carry_num == 3 && part_move_num == 1 {
                droppers.push(creep);
            }
        }
        if !droppers.is_empty() {
            Some(droppers)
        } else {
            None
        }
    } else {
        None
    }
}

pub fn select_workers() -> Option<Vec<Creep>> {
    let creeps = get_objects_by_prototype(prototypes::CREEP);
    if !creeps.is_empty() {
        let mut workers = Vec::new();
        for creep in creeps {
            let mut part_work_num = 0;
            let mut part_carry_num = 0;
            let mut part_move_num = 0;
            for body_part in creep.body() {
                match body_part.part() {
                    Part::Work => part_work_num += 1,
                    Part::Carry => part_carry_num += 1,
                    Part::Move => part_move_num += 1,
                    _ => (),
                }
            }
            if part_work_num == 3 && part_carry_num == 4 && part_move_num == 6 {
                workers.push(creep);
            }
        }
        if !workers.is_empty() {
            Some(workers)
        } else {
            None
        }
    } else {
        None
    }
}

pub fn select_mages() -> Option<Vec<Creep>> {
    let creeps = get_objects_by_prototype(prototypes::CREEP);
    if !creeps.is_empty() {
        let mut mages = Vec::new();
        for creep in creeps {
            let mut part_move_num = 0;
            let mut part_ranged_attack_num = 0;
            let mut part_heal_num = 0;
            for body_part in creep.body() {
                match body_part.part() {
                    Part::Move => part_move_num += 1,
                    Part::RangedAttack => part_ranged_attack_num += 1,
                    Part::Heal => part_heal_num += 1,
                    _ => (),
                }
            }
            if part_move_num == 6 && part_ranged_attack_num == 3 && part_heal_num == 1 {
                mages.push(creep);
            }
        }
        if !mages.is_empty() {
            Some(mages)
        } else {
            None
        }
    } else {
        None
    }
}

pub fn select_ers() -> Option<Vec<Creep>> {
    let creeps = get_objects_by_prototype(prototypes::CREEP);
    if !creeps.is_empty() {
        let mut ers = Vec::new();
        for creep in creeps {
            let mut part_tough_num = 0;
            for body_part in creep.body() {
                match body_part.part() {
                    Part::Tough => part_tough_num += 1,
                    _ => (),
                }
            }
            if part_tough_num == 1 {
                ers.push(creep);
            }
        }
        if !ers.is_empty() {
            Some(ers)
        } else {
            None
        }
    } else {
        None
    }
}

pub fn select_enemy_creeps() -> Option<Vec<GameObject>> {
    let creeps = get_objects_by_prototype(prototypes::CREEP);
    if !creeps.is_empty() {
        let mut enemy_creeps = Vec::new();
        for creep in creeps {
            if !creep.my() {
                enemy_creeps.push(creep.into());
            }
        }
        if !enemy_creeps.is_empty() {
            Some(enemy_creeps)
        } else {
            None
        }
    } else {
        None
    }
}
