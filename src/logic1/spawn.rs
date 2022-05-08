use std::collections::HashMap;

use js_sys::{Map, Object};
use log::warn;
use screeps_arena::{
    game::utils::{self, create_construction_site, get_terrain_at},
    prototypes::{self, PrototypeConstant},
    GameObject,
    ResourceType::Energy,
    StructureSpawn, Terrain,
    constants::Part,
};
use wasm_bindgen::{JsCast, JsValue};

use crate::utils::{
    find::{find_closest_by_range, self},
    move_to_do::{move_to_pickup, move_to_transfer, move_to_withdraw, Action},
    select::{self, select_creeps::select_carriers, select_resource, select_structure},
};

pub fn spawn() {
    build_wall();
    build_rampart_for_spawn();
    build_entension();

    let my_spawn = select::select_structure::select_my_spawn().unwrap();
    let enemy_spawn = select::select_structure::select_enemy_spawn().unwrap();
    let enemy_creeps = select::select_creeps::select_enemy_creeps();
    let carriers = select::select_creeps::select_carriers();
    let workers = select::select_creeps::select_workers();
    let droppers = select::select_creeps::select_droppers();
    let mages = select::select_creeps::select_mages();
    let ers = select::select_creeps::select_ers();
    let full_containers = select::select_structure::select_full_containers();
    let my_spawn_enemy_creeps_closest = find::find_closest_by_range(&my_spawn, &enemy_creeps);

    // 体型数据
    let carrier_body = [Part::Move, Part::Carry, Part::Move, Part::Carry];
    let dropper_body = [Part::Carry, Part::Carry, Part::Carry, Part::Move];
    let mages_body = [Part::Move, Part::Move, Part::Move, Part::Move, Part::Move, Part::Move, Part::RangedAttack, Part::RangedAttack, Part::RangedAttack, Part::Heal];
    let workers_body = [Part::Work, Part::Work, Part::Work, Part::Carry, Part::Carry, Part::Carry, Part::Carry, Part::Move, Part::Move, Part::Move, Part::Move, Part::Move, Part::Move];
    let ers_body = [Part::Tough];

    let fight_time = 260;

    // 出生顺序管理
    
}

fn build_wall() {
    let my_spawn = select::select_structure::select_my_spawn();
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
            let y = y as f64;
            for x in 1u8..13u8 {
                let x = x as f64;
                let position = Map::new();
                position.set(
                    JsValue::from_str("x").as_ref(),
                    JsValue::from_f64(x).as_ref(),
                );
                position.set(
                    JsValue::from_str("y").as_ref(),
                    JsValue::from_f64(y).as_ref(),
                );
                if get_terrain_at(position.as_ref()) != Terrain::Wall {
                    ys.push(y);
                }
            }
        }

        // 找出出现次数最少的y
        let ys_ord: Vec<u8> = ys.iter().map(|y| y.to_owned() as u8).collect();
        let mut hash_map: HashMap<u8, u8> = HashMap::new();
        for y_ord in ys_ord {
            if hash_map.contains_key(&y_ord) {
                if let Some(x) = hash_map.get_mut(&y_ord) {
                    *x += 1;
                }
                // hash_map[y_ord] += 1;
            } else {
                hash_map.insert(y_ord, 1);
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
        let mut wall_line = min_exist_ys[0];
        for min_exist_y in min_exist_ys {
            let my_spawn_to_min_exist_y = my_spawn
                .as_ref()
                .unwrap()
                .y()
                .checked_sub(min_exist_y)
                .unwrap_or(min_exist_y - my_spawn.as_ref().unwrap().y());
            let my_spawn_to_wall_line = my_spawn
                .as_ref()
                .unwrap()
                .y()
                .checked_sub(wall_line)
                .unwrap_or(wall_line - my_spawn.as_ref().unwrap().y());
            if my_spawn_to_min_exist_y < my_spawn_to_wall_line {
                wall_line = min_exist_y;
            }
        }

        // 安放建筑工地
        for x in 1u8..13u8 {
            let x_f64 = x as f64;
            let wall_line_f64 = wall_line as f64;
            let position = Map::new();
            position.set(
                JsValue::from_str("x").as_ref(),
                JsValue::from_f64(x_f64).as_ref(),
            );
            position.set(
                JsValue::from_str("y").as_ref(),
                JsValue::from_f64(wall_line_f64).as_ref(),
            );
            if get_terrain_at(position.as_ref()) != Terrain::Wall {
                if create_construction_site(x, wall_line, prototypes::STRUCTURE_WALL.prototype())
                    .is_err()
                {
                    warn!("{}", "无法创建wall工地");
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
            let y = y as f64;
            for x in 86u8..98u8 {
                let x = x as f64;
                let position = Map::new();
                position.set(
                    JsValue::from_str("x").as_ref(),
                    JsValue::from_f64(x).as_ref(),
                );
                position.set(
                    JsValue::from_str("y").as_ref(),
                    JsValue::from_f64(y).as_ref(),
                );
                if get_terrain_at(position.as_ref()) != Terrain::Wall {
                    ys.push(y);
                }
            }
        }

        // 找出出现次数最少的y
        let ys_ord: Vec<u8> = ys.iter().map(|y| y.to_owned() as u8).collect();
        let mut hash_map: HashMap<u8, u8> = HashMap::new();
        for y_ord in ys_ord {
            if hash_map.contains_key(&y_ord) {
                if let Some(x) = hash_map.get_mut(&y_ord) {
                    *x += 1;
                }
                // hash_map[y_ord] += 1;
            } else {
                hash_map.insert(y_ord, 1);
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
        let mut wall_line = min_exist_ys[0];
        for min_exist_y in min_exist_ys {
            let my_spawn_to_min_exist_y = my_spawn
                .as_ref()
                .unwrap()
                .y()
                .checked_sub(min_exist_y)
                .unwrap_or(min_exist_y - my_spawn.as_ref().unwrap().y());
            let my_spawn_to_wall_line = my_spawn
                .as_ref()
                .unwrap()
                .y()
                .checked_sub(wall_line)
                .unwrap_or(wall_line - my_spawn.as_ref().unwrap().y());
            if my_spawn_to_min_exist_y < my_spawn_to_wall_line {
                wall_line = min_exist_y;
            }
        }

        // 安放建筑工地
        for x in 86u8..98u8 {
            let x_f64 = x as f64;
            let wall_line_f64 = wall_line as f64;
            let position = Map::new();
            position.set(
                JsValue::from_str("x").as_ref(),
                JsValue::from_f64(x_f64).as_ref(),
            );
            position.set(
                JsValue::from_str("y").as_ref(),
                JsValue::from_f64(wall_line_f64).as_ref(),
            );
            if get_terrain_at(position.as_ref()) != Terrain::Wall {
                if create_construction_site(x, wall_line, prototypes::STRUCTURE_WALL.prototype())
                    .is_err()
                {
                    warn!("{}", "无法创建wall工地");
                }
            }
        }
    }
}

fn build_tower() {
    let out_containers = select::select_structure::select_out_containers();
    if out_containers.is_some() {
        let out_containers = out_containers.unwrap();
        for out_container in out_containers {
            if create_construction_site(
                out_container.x(),
                out_container.y() - 1,
                prototypes::STRUCTURE_TOWER.prototype(),
            )
            .is_err()
            {
                warn!("{}", "无法创建tower工地");
            }
        }
    }
}

fn build_entension() {
    if utils::get_ticks() >= 240 {
        let out_containers = select::select_structure::select_out_containers();
        if out_containers.is_some() {
            let out_containers = out_containers.unwrap();
            for out_container in out_containers {
                if create_construction_site(
                    out_container.x() - 3,
                    out_container.y() - 3,
                    prototypes::STRUCTURE_EXTENSION.prototype(),
                )
                .is_err()
                {
                    warn!("{}", "无法创建extension工地");
                }

                if create_construction_site(
                    out_container.x() - 3,
                    out_container.y() + 3,
                    prototypes::STRUCTURE_EXTENSION.prototype(),
                )
                .is_err()
                {
                    warn!("{}", "无法创建extension工地");
                }

                if create_construction_site(
                    out_container.x() - 3,
                    out_container.y(),
                    prototypes::STRUCTURE_EXTENSION.prototype(),
                )
                .is_err()
                {
                    warn!("{}", "无法创建extension工地");
                }

                if create_construction_site(
                    out_container.x(),
                    out_container.y() - 3,
                    prototypes::STRUCTURE_EXTENSION.prototype(),
                )
                .is_err()
                {
                    warn!("{}", "无法创建extension工地");
                }

                if create_construction_site(
                    out_container.x(),
                    out_container.y() + 3,
                    prototypes::STRUCTURE_EXTENSION.prototype(),
                )
                .is_err()
                {
                    warn!("{}", "无法创建extension工地");
                }

                if create_construction_site(
                    out_container.x() + 3,
                    out_container.y() + 3,
                    prototypes::STRUCTURE_EXTENSION.prototype(),
                )
                .is_err()
                {
                    warn!("{}", "无法创建extension工地");
                }

                if create_construction_site(
                    out_container.x() + 3,
                    out_container.y() - 3,
                    prototypes::STRUCTURE_EXTENSION.prototype(),
                )
                .is_err()
                {
                    warn!("{}", "无法创建extension工地");
                }

                if create_construction_site(
                    out_container.x() + 3,
                    out_container.y(),
                    prototypes::STRUCTURE_EXTENSION.prototype(),
                )
                .is_err()
                {
                    warn!("{}", "无法创建extension工地");
                }
            }
        }
    }
}

fn build_rampart_for_spawn() {
    let my_spawn = select::select_structure::select_my_spawn().unwrap();
    if create_construction_site(my_spawn.x(), my_spawn.y(), prototypes::STRUCTURE_RAMPART.prototype()).is_err() {
        warn!("{}", "无法创建城墙");
    }
}