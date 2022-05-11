use std::collections::HashMap;

use js_sys::{Map, Object};
use log::warn;
use screeps_arena::{
    constants::Part,
    game::utils::{self, create_construction_site, get_terrain_at, get_ticks},
    prototypes::{self, PrototypeConstant},
    StructureSpawn, Terrain,
};
use wasm_bindgen::{JsCast, JsValue};

use crate::utils::{
    find::{self},
    select::{self},
};

pub fn spawn() -> (bool, u8) {
    // let mut is_close = false;

    let mut wall_line_mut: u8 = 0;
    if get_ticks() == 1 {
        wall_line_mut = build_wall();
        build_rampart_for_spawn();
        build_extensions();
        build_towers();
    }

    let my_spawn = select::select_structure::select_my_spawn()
        .unwrap()
        .unchecked_into::<StructureSpawn>();
    // let enemy_spawn = select::select_structure::select_enemy_spawn().unwrap();
    let enemy_creeps = select::select_creeps::select_enemy_creeps();
    let carriers = select::select_creeps::select_carriers();
    let workers = select::select_creeps::select_workers();
    let droppers = select::select_creeps::select_droppers();
    let mages = select::select_creeps::select_mages();
    let ers = select::select_creeps::select_ers();
    // let full_containers = select::select_structure::select_full_containers();
    let my_spawn_enemy_creeps_closest = find::find_closest_by_range(&my_spawn, &enemy_creeps);

    // 体型数据
    let carriers_body = [Part::Move, Part::Carry, Part::Move, Part::Carry];
    let droppers_body = [Part::Carry, Part::Carry, Part::Carry, Part::Move];
    let mages_body = [
        Part::Move,
        Part::Move,
        Part::Move,
        Part::Move,
        Part::Move,
        Part::Move,
        Part::RangedAttack,
        Part::RangedAttack,
        Part::RangedAttack,
        Part::Heal,
    ];
    let workers_body = [
        Part::Work,
        Part::Work,
        Part::Work,
        Part::Carry,
        Part::Carry,
        Part::Carry,
        Part::Carry,
        Part::Move,
        Part::Move,
        Part::Move,
        Part::Move,
        Part::Move,
        Part::Move,
    ];
    let ers_body = [Part::Tough];

    // let fight_time = 260;

    // 出生顺序管理
    if utils::get_ticks() <= 500 {
        if carriers
            .unwrap_or_else(|| {
                warn!("{}", "carrier数量为零");
                Vec::new()
            })
            .len()
            < 2
        {
            // let carriers = carriers.unwrap();
            match my_spawn.spawn_creep(&carriers_body) {
                Ok(creep) => warn!("carrier: {}", creep.id()),
                Err(err) => warn!("carrier: {:?}", err),
            }
        } else if workers
            .unwrap_or_else(|| {
                warn!("{}", "worker数量为零");
                Vec::new()
            })
            .len()
            < 1
        {
            // let workers = workers.unwrap();
            match my_spawn.spawn_creep(&workers_body) {
                Ok(creep) => warn!("worker: {}", creep.id()),
                Err(err) => warn!("worker: {:?}", err),
            }
        } else if droppers
            .unwrap_or_else(|| {
                warn!("{}", "dropper数量为零");
                Vec::new()
            })
            .len()
            < 1
        {
            // let droppers = droppers.unwrap();
            match my_spawn.spawn_creep(&droppers_body) {
                Ok(creep) => warn!("dropper: {}", creep.id()),
                Err(err) => warn!("dropper: {:?}", err),
            }
        } else if ers
            .unwrap_or_else(|| {
                warn!("{}", "er数量为零");
                Vec::new()
            })
            .len()
            < 1
        {
            // let ers = ers.unwrap();
            match my_spawn.spawn_creep(&ers_body) {
                Ok(creep) => warn!("er: {}", creep.id()),
                Err(err) => warn!("er: {:?}", err),
            }
        } else if mages
            .unwrap_or_else(|| {
                warn!("{}", "mage数量为零");
                Vec::new()
            })
            .len()
            < 99
        {
            // let mages = mages.unwrap();
            match my_spawn.spawn_creep(&mages_body) {
                Ok(creep) => warn!("mage: {}", creep.id()),
                Err(err) => warn!("mage: {:?}", err),
            }
        }
    } else {
        if carriers
            .unwrap_or_else(|| {
                warn!("{}", "carrier数量为零");
                Vec::new()
            })
            .len()
            < 2
        {
            // let carriers = carriers.unwrap();
            match my_spawn.spawn_creep(&carriers_body) {
                Ok(creep) => warn!("carrier: {}", creep.id()),
                Err(err) => warn!("carrier: {:?}", err),
            }
        } else if mages
            .unwrap_or_else(|| {
                warn!("{}", "mage数量为零");
                Vec::new()
            })
            .len()
            < 99
        {
            // let mages = mages.unwrap();
            match my_spawn.spawn_creep(&mages_body) {
                Ok(creep) => warn!("mage: {}", creep.id()),
                Err(err) => warn!("mage: {:?}", err),
            }
        }
    }

    if enemy_creeps.is_some() {
        if my_spawn.x() < 50 {
            if wall_line_mut > my_spawn.y() {
                if my_spawn_enemy_creeps_closest.as_ref().unwrap().x() >= my_spawn.x() - 4
                    && my_spawn_enemy_creeps_closest.as_ref().unwrap().x() <= my_spawn.x() + 7
                    && my_spawn_enemy_creeps_closest.as_ref().unwrap().y() >= my_spawn.y() - 45
                    && my_spawn_enemy_creeps_closest.as_ref().unwrap().y() <= wall_line_mut
                {
                    (true, wall_line_mut)
                } else {
                    (false, wall_line_mut)
                }
            } else {
                if my_spawn_enemy_creeps_closest.as_ref().unwrap().x() >= my_spawn.x() - 4
                    && my_spawn_enemy_creeps_closest.as_ref().unwrap().x() <= my_spawn.x() + 7
                    && my_spawn_enemy_creeps_closest.as_ref().unwrap().y() >= wall_line_mut
                    && my_spawn_enemy_creeps_closest.as_ref().unwrap().y() <= my_spawn.y() + 54
                {
                    (true, wall_line_mut)
                } else {
                    (false, wall_line_mut)
                }
            }
        } else if my_spawn.x() > 50 {
            if wall_line_mut > my_spawn.y() {
                if my_spawn_enemy_creeps_closest.as_ref().unwrap().x() > my_spawn.x() - 7
                    && my_spawn_enemy_creeps_closest.as_ref().unwrap().x() <= my_spawn.x() + 4
                    && my_spawn_enemy_creeps_closest.as_ref().unwrap().y() >= my_spawn.y() - 54
                    && my_spawn_enemy_creeps_closest.as_ref().unwrap().y() <= wall_line_mut
                {
                    (true, wall_line_mut)
                } else {
                    (false, wall_line_mut)
                }
            } else {
                if my_spawn_enemy_creeps_closest.as_ref().unwrap().x() > my_spawn.x() - 7
                    && my_spawn_enemy_creeps_closest.as_ref().unwrap().x() <= my_spawn.x() + 4
                    && my_spawn_enemy_creeps_closest.as_ref().unwrap().y() >= wall_line_mut
                    && my_spawn_enemy_creeps_closest.as_ref().unwrap().y() <= my_spawn.y() + 45
                {
                    (true, wall_line_mut)
                } else {
                    (false, wall_line_mut)
                }
            }
        } else {
            (false, wall_line_mut)
        }
    } else {
        (false, wall_line_mut)
    }
}

fn build_wall() -> u8 {
    let my_spawn = select::select_structure::select_my_spawn();
    let mut wall_line_mut: u8 = 0;
    if my_spawn
        .as_ref()
        .unwrap()
        .unchecked_ref::<StructureSpawn>()
        .x()
        < 50
    {
        // 扫描区域，把不是墙的块的y坐标加入vec
        let mut ys = Vec::new();
        for y in 21u8..78u8 {
            // let y = y as f64;
            for x in 1u8..13u8 {
                // let x = x as f64;
                let position = Map::new();
                position.set(JsValue::from("x").as_ref(), JsValue::from(x).as_ref());
                position.set(JsValue::from("y").as_ref(), JsValue::from(y).as_ref());
                let position_object = Object::from_entries(position.as_ref()).unwrap();
                if get_terrain_at(&position_object) != Terrain::Wall {
                    ys.push(y);
                }
            }
        }
        // warn!("{:?}", ys);

        // 找出出现次数最少的y
        // let ys_ord: Vec<u8> = ys.iter().map(|y| y.to_owned() as u8).collect();
        let mut hash_map: HashMap<u8, u8> = HashMap::new();
        for y in ys {
            if hash_map.contains_key(&y) {
                if let Some(x) = hash_map.get_mut(&y) {
                    *x += 1;
                }
                // hash_map[y_ord] += 1;
            } else {
                hash_map.insert(y, 1);
            }
        }
        // warn!("{:?}", hash_map);
        let mut hash_map_value: Vec<u8> = hash_map.clone().into_values().collect();
        hash_map_value.sort();
        let mut min_exist_ys: Vec<u8> = Vec::new();
        for kv in hash_map {
            if kv.1 == hash_map_value[0] {
                min_exist_ys.push(kv.0);
            }
        }
        // warn!("{:?}", min_exist_ys);

        // 在所有出现次数最少的y中，距离母巢最近的y作为墙
        wall_line_mut = min_exist_ys[0];
        for min_exist_y in min_exist_ys {
            let my_spawn_to_min_exist_y = my_spawn
                .as_ref()
                .unwrap()
                .y()
                .checked_sub(min_exist_y)
                .unwrap_or(min_exist_y - my_spawn.as_ref().unwrap().y());
            let my_spawn_to_wall_line_mut = my_spawn
                .as_ref()
                .unwrap()
                .y()
                .checked_sub(wall_line_mut)
                .unwrap_or(wall_line_mut - my_spawn.as_ref().unwrap().y());
            if my_spawn_to_min_exist_y < my_spawn_to_wall_line_mut {
                wall_line_mut = min_exist_y;
            }
        }

        // 安放建筑工地
        for x in 1u8..13u8 {
            // let x_f64 = x as f64;
            // let wall_line_mut_f64 = wall_line_mut as f64;
            let position = Map::new();
            position.set(JsValue::from("x").as_ref(), JsValue::from(x).as_ref());
            position.set(
                JsValue::from("y").as_ref(),
                JsValue::from(wall_line_mut).as_ref(),
            );
            let position_object = Object::from_entries(position.as_ref()).unwrap();
            if get_terrain_at(&position_object) != Terrain::Wall {
                match create_construction_site(
                    x,
                    wall_line_mut,
                    prototypes::STRUCTURE_WALL.prototype(),
                ) {
                    Ok(construction_site) => {
                        warn!("Wall: {}: {}", construction_site.x(), construction_site.y())
                    }
                    Err(err) => warn!("Wall: {:?}", err),
                }
            }
        }
    } else if my_spawn
        .as_ref()
        .unwrap()
        .unchecked_ref::<StructureSpawn>()
        .x()
        > 50
    {
        let mut ys = Vec::new();
        for y in 21u8..78u8 {
            // let y = y as f64;
            for x in 86u8..98u8 {
                // let x = x as f64;
                let position = Map::new();
                position.set(JsValue::from("x").as_ref(), JsValue::from(x).as_ref());
                position.set(JsValue::from("y").as_ref(), JsValue::from(y).as_ref());
                let position_object = Object::from_entries(position.as_ref()).unwrap();
                if get_terrain_at(&position_object) != Terrain::Wall {
                    ys.push(y);
                }
            }
        }

        // 找出出现次数最少的y
        // let ys_ord: Vec<u8> = ys.iter().map(|y| y.to_owned() as u8).collect();
        let mut hash_map: HashMap<u8, u8> = HashMap::new();
        for y in ys {
            if hash_map.contains_key(&y) {
                if let Some(x) = hash_map.get_mut(&y) {
                    *x += 1;
                }
                // hash_map[y_ord] += 1;
            } else {
                hash_map.insert(y, 1);
            }
        }
        let mut hash_map_value: Vec<u8> = hash_map.clone().into_values().collect();
        hash_map_value.sort();
        let mut min_exist_ys: Vec<u8> = Vec::new();
        for kv in hash_map {
            if kv.1 == hash_map_value[0] {
                min_exist_ys.push(kv.0);
            }
        }

        // 在所有出现次数最少的y中，距离母巢最近的y作为墙
        wall_line_mut = min_exist_ys[0];
        for min_exist_y in min_exist_ys {
            let my_spawn_to_min_exist_y = my_spawn
                .as_ref()
                .unwrap()
                .y()
                .checked_sub(min_exist_y)
                .unwrap_or(min_exist_y - my_spawn.as_ref().unwrap().y());
            let my_spawn_to_wall_line_mut = my_spawn
                .as_ref()
                .unwrap()
                .y()
                .checked_sub(wall_line_mut)
                .unwrap_or(wall_line_mut - my_spawn.as_ref().unwrap().y());
            if my_spawn_to_min_exist_y < my_spawn_to_wall_line_mut {
                wall_line_mut = min_exist_y;
            }
        }

        // 安放建筑工地
        for x in 86u8..98u8 {
            // let x_f64 = x as f64;
            // let wall_line_mut_f64 = wall_line_mut as f64;
            let position = Map::new();
            position.set(JsValue::from("x").as_ref(), JsValue::from(x).as_ref());
            position.set(
                JsValue::from("y").as_ref(),
                JsValue::from(wall_line_mut).as_ref(),
            );
            let position_object = Object::from_entries(position.as_ref()).unwrap();
            if get_terrain_at(&position_object) != Terrain::Wall {
                match create_construction_site(
                    x,
                    wall_line_mut,
                    prototypes::STRUCTURE_WALL.prototype(),
                ) {
                    Ok(construction_site) => {
                        warn!("Wall: {}: {}", construction_site.x(), construction_site.y())
                    }
                    Err(err) => warn!("Wall: {:?}", err),
                }
            }
        }
    }
    wall_line_mut
}

fn build_towers() {
    let out_containers = select::select_structure::select_out_containers();
    if out_containers.is_some() {
        let out_containers = out_containers.unwrap();
        for out_container in out_containers {
            match create_construction_site(
                out_container.x(),
                out_container.y() - 1,
                prototypes::STRUCTURE_TOWER.prototype(),
            ) {
                Ok(construction_site) => {
                    warn!(
                        "Tower: {}: {}",
                        construction_site.x(),
                        construction_site.y()
                    )
                }
                Err(err) => warn!("Tower: {:?}", err),
            }
        }
    }
}

fn build_extensions() {
    if utils::get_ticks() >= 240 {
        let out_containers = select::select_structure::select_out_containers();
        if out_containers.is_some() {
            let out_containers = out_containers.unwrap();
            for out_container in out_containers {
                match create_construction_site(
                    out_container.x() - 3,
                    out_container.y() - 3,
                    prototypes::STRUCTURE_EXTENSION.prototype(),
                ) {
                    Ok(construction_site) => {
                        warn!(
                            "Extension: {}: {}",
                            construction_site.x(),
                            construction_site.y()
                        )
                    }
                    Err(err) => warn!("Extension: {:?}", err),
                }

                match create_construction_site(
                    out_container.x() - 3,
                    out_container.y() + 3,
                    prototypes::STRUCTURE_EXTENSION.prototype(),
                ) {
                    Ok(construction_site) => {
                        warn!(
                            "Extension: {}: {}",
                            construction_site.x(),
                            construction_site.y()
                        )
                    }
                    Err(err) => warn!("Extension: {:?}", err),
                }

                match create_construction_site(
                    out_container.x() - 3,
                    out_container.y(),
                    prototypes::STRUCTURE_EXTENSION.prototype(),
                ) {
                    Ok(construction_site) => {
                        warn!(
                            "Extension: {}: {}",
                            construction_site.x(),
                            construction_site.y()
                        )
                    }
                    Err(err) => warn!("Extension: {:?}", err),
                }

                match create_construction_site(
                    out_container.x(),
                    out_container.y() - 3,
                    prototypes::STRUCTURE_EXTENSION.prototype(),
                ) {
                    Ok(construction_site) => {
                        warn!(
                            "Extension: {}: {}",
                            construction_site.x(),
                            construction_site.y()
                        )
                    }
                    Err(err) => warn!("Extension: {:?}", err),
                }

                match create_construction_site(
                    out_container.x(),
                    out_container.y() + 3,
                    prototypes::STRUCTURE_EXTENSION.prototype(),
                ) {
                    Ok(construction_site) => {
                        warn!(
                            "Extension: {}: {}",
                            construction_site.x(),
                            construction_site.y()
                        )
                    }
                    Err(err) => warn!("Extension: {:?}", err),
                }

                match create_construction_site(
                    out_container.x() + 3,
                    out_container.y() + 3,
                    prototypes::STRUCTURE_EXTENSION.prototype(),
                ) {
                    Ok(construction_site) => {
                        warn!(
                            "Extension: {}: {}",
                            construction_site.x(),
                            construction_site.y()
                        )
                    }
                    Err(err) => warn!("Extension: {:?}", err),
                }

                match create_construction_site(
                    out_container.x() + 3,
                    out_container.y() - 3,
                    prototypes::STRUCTURE_EXTENSION.prototype(),
                ) {
                    Ok(construction_site) => {
                        warn!(
                            "Extension: {}: {}",
                            construction_site.x(),
                            construction_site.y()
                        )
                    }
                    Err(err) => warn!("Extension: {:?}", err),
                }

                match create_construction_site(
                    out_container.x() + 3,
                    out_container.y(),
                    prototypes::STRUCTURE_EXTENSION.prototype(),
                ) {
                    Ok(construction_site) => {
                        warn!(
                            "Extension: {}: {}",
                            construction_site.x(),
                            construction_site.y()
                        )
                    }
                    Err(err) => warn!("Extension: {:?}", err),
                }
            }
        }
    }
}

fn build_rampart_for_spawn() {
    let my_spawn = select::select_structure::select_my_spawn().unwrap();
    match create_construction_site(
        my_spawn.x(),
        my_spawn.y(),
        prototypes::STRUCTURE_RAMPART.prototype(),
    ) {
        Ok(construction_site) => warn!(
            "Rampart: {}: {}",
            construction_site.x(),
            construction_site.y()
        ),
        Err(err) => warn!("Rampart: {:?}", err),
    }
}
