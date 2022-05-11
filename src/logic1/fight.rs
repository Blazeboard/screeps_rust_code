use js_sys::{Map, Object};

use screeps_arena::{
    game::{
        pathfinder::{CostMatrix, FindPathOptions},
        utils::{self, find_path, get_direction, get_ticks},
    },
    prelude::*,
    Creep, Direction, GameObject, StructureExtension, StructureSpawn, StructureTower,
};
use wasm_bindgen::{JsCast, JsValue};

use crate::utils::{
    find::{self, find_in_x_range},
    select::{
        self,
        select_creeps::{self, select_my_creeps_not_me},
        select_structure,
    },
    utils::{find_lowest_hits_from_array, is_near_to_teammates, set_obstacles},
};

pub fn fight((is_close, wall_line): (bool, u8)) {
    let my_spawn = select_structure::select_my_spawn().unwrap();
    let enemy_creeps = select_creeps::select_enemy_creeps();
    let enemy_spawn = select_structure::select_enemy_spawn().unwrap();
    let enemy_extensions = select_structure::select_enemy_extensions();
    let enemy_towers = select_structure::select_enemy_towers();
    let mages = select_creeps::select_mages();
    let my_injured_creeps = select_creeps::select_my_injured_creeps();

    let my_spawn_enemy_creeps_closest = find::find_closest_by_range(&my_spawn, &enemy_creeps);
    let team_position = Map::new();
    let mut costs = CostMatrix::new();
    let mut team_leader_go_direction: Option<Direction> = None;

    costs = set_obstacles(costs);

    let mut team_map: Vec<(Creep, bool)> = Vec::new();

    // 设置集结点
    let mut team_position_object = Object::new();
    if my_spawn.x() < 50 {
        if wall_line > my_spawn.y() {
            team_position.set(&JsValue::from("x"), &JsValue::from(my_spawn.x() + 2));
            team_position.set(&JsValue::from("y"), &JsValue::from(my_spawn.y() - 13));
            team_position_object = Object::from_entries(team_position.as_ref()).unwrap();
        } else {
            team_position.set(&JsValue::from("x"), &JsValue::from(my_spawn.x() + 2));
            team_position.set(&JsValue::from("y"), &JsValue::from(my_spawn.y() + 13));
            team_position_object = Object::from_entries(team_position.as_ref()).unwrap();
        }
    } else {
        if wall_line > my_spawn.y() {
            team_position.set(&JsValue::from("x"), &JsValue::from(my_spawn.x() - 2));
            team_position.set(&JsValue::from("y"), &JsValue::from(my_spawn.y() - 13));
            team_position_object = Object::from_entries(team_position.as_ref()).unwrap();
        } else {
            team_position.set(&JsValue::from("x"), &JsValue::from(my_spawn.x() - 2));
            team_position.set(&JsValue::from("y"), &JsValue::from(my_spawn.y() + 13));
            team_position_object = Object::from_entries(team_position.as_ref()).unwrap();
        }
    }

    // 组队
    if mages.is_some() && enemy_creeps.is_some() {
        let mages = mages.as_ref().unwrap();
        if mages.len() >= 2 {
            for mage in mages {
                let mage_enemy_creeps_in_3_range = find_in_x_range(mage.as_ref(), &enemy_creeps, 3);
                let teammates = select_my_creeps_not_me(&mage);
                if mage_enemy_creeps_in_3_range.is_some() && is_near_to_teammates(&mage, &teammates)
                {
                    team_map.push((mage.clone(), false));
                }
            }

            // 挑选队长
            let team_map_iter = team_map.clone();
            for num in 0..team_map_iter.len() {
                if team_map[num].1 == true {
                    team_map.push((team_map[num].0.clone(), false));
                    // team_map.retain(|teammate| teammate.1 == false);
                    team_map.remove(num);
                }
            }
            if team_map.len() >= 2 {
                let closest_enemy =
                    find::find_closest_by_range(&team_map[0].0, &enemy_creeps).unwrap();
                let mut team_map_vec: Vec<GameObject> = Vec::new();
                // let team_map_iter = team_map.clone();
                for team_map_item in team_map.clone() {
                    team_map_vec.push(team_map_item.0.into());
                }
                let closest_teammate_to_enemy =
                    find::find_closest_by_range(&closest_enemy, &Some(team_map_vec))
                        .unwrap()
                        .unchecked_into::<Creep>();
                let team_map_iter = team_map.clone();
                for num in 0..team_map_iter.len() {
                    if team_map[num].0.id() == closest_teammate_to_enemy.id() {
                        team_map.push((closest_teammate_to_enemy.clone(), true));
                        team_map.remove(num);
                    }
                }
            }
        }
    }

    // 队长移动逻辑
    for teammate in &team_map {
        if teammate.1 == true {
            let team_leader = &teammate.0;
            if get_ticks() < 260 {
                // 260之后是战争状态
                team_leader.move_to(&team_position_object, None);
            } else {
                if enemy_creeps.is_some() {
                    let team_leader_enemy_creeps_closest =
                        find::find_closest_by_range(team_leader.as_ref(), &enemy_creeps);

                    if utils::get_range(
                        team_leader.unchecked_ref(),
                        team_leader_enemy_creeps_closest
                            .as_ref()
                            .unwrap()
                            .unchecked_ref(),
                    ) >= 4
                        && utils::get_range(
                            team_leader.unchecked_ref(),
                            enemy_spawn.unchecked_ref(),
                        ) > 7
                    {
                        let team_leader_team_leader_enemy_closest_path = find_path(
                            team_leader.unchecked_ref(),
                            team_leader_enemy_creeps_closest.unwrap().unchecked_ref(),
                            None,
                        );
                        let team_leader_team_leader_enemy_closest_path_js_map = Map::new();
                        team_leader_team_leader_enemy_closest_path_js_map.set(
                            &JsValue::from("x"),
                            &JsValue::from(team_leader_team_leader_enemy_closest_path.path()[0].x),
                        );
                        team_leader_team_leader_enemy_closest_path_js_map.set(
                            &JsValue::from("y"),
                            &JsValue::from(team_leader_team_leader_enemy_closest_path.path()[0].y),
                        );
                        let team_leader_team_leader_enemy_closest_path_js_map_object =
                            Object::from_entries(
                                team_leader_team_leader_enemy_closest_path_js_map.as_ref(),
                            )
                            .unwrap();
                        team_leader.move_to(
                            &team_leader_team_leader_enemy_closest_path_js_map_object,
                            None,
                        );

                        team_leader_go_direction = Some(get_direction(
                            team_leader_team_leader_enemy_closest_path.path()[0].x
                                - team_leader.x(),
                            team_leader_team_leader_enemy_closest_path.path()[0].y
                                - team_leader.y(),
                        ));
                    } else if utils::get_range(
                        team_leader.unchecked_ref(),
                        team_leader_enemy_creeps_closest
                            .as_ref()
                            .unwrap()
                            .unchecked_ref(),
                    ) == 3
                        && utils::get_range(
                            team_leader.unchecked_ref(),
                            enemy_spawn.unchecked_ref(),
                        ) > 7
                    {
                        team_leader_go_direction = None;
                    } else if utils::get_range(
                        team_leader.unchecked_ref(),
                        team_leader_enemy_creeps_closest
                            .as_ref()
                            .unwrap()
                            .unchecked_ref(),
                    ) < 3
                        && utils::get_range(
                            team_leader.unchecked_ref(),
                            enemy_spawn.unchecked_ref(),
                        ) > 7
                    {
                        let team_leader_team_leader_enemy_closest_path = find_path(
                            team_leader.unchecked_ref(),
                            team_leader_enemy_creeps_closest.unwrap().unchecked_ref(),
                            {
                                let fpo = FindPathOptions::new();
                                fpo.cost_matrix(&costs);
                                fpo.flee(true);
                                Some(fpo).as_ref()
                            },
                        );
                        let team_leader_team_leader_enemy_closest_path_js_map = Map::new();
                        team_leader_team_leader_enemy_closest_path_js_map.set(
                            &JsValue::from("x"),
                            &JsValue::from(team_leader_team_leader_enemy_closest_path.path()[0].x),
                        );
                        team_leader_team_leader_enemy_closest_path_js_map.set(
                            &JsValue::from("y"),
                            &JsValue::from(team_leader_team_leader_enemy_closest_path.path()[0].y),
                        );
                        let team_leader_team_leader_enemy_closest_path_js_map_object =
                            Object::from_entries(
                                team_leader_team_leader_enemy_closest_path_js_map.as_ref(),
                            )
                            .unwrap();
                        team_leader.move_to(
                            &team_leader_team_leader_enemy_closest_path_js_map_object,
                            None,
                        );

                        team_leader_go_direction = Some(get_direction(
                            team_leader_team_leader_enemy_closest_path.path()[0].x
                                - team_leader.x(),
                            team_leader_team_leader_enemy_closest_path.path()[0].y
                                - team_leader.y(),
                        ));
                    } else {
                        team_leader.move_to(enemy_spawn.as_ref(), None);
                    }
                } else {
                    team_leader.move_to(enemy_spawn.as_ref(), None);
                }
            }
        }
    }

    // 队员移动逻辑
    for teammate in &team_map {
        if teammate.1 == false {
            if get_ticks() < 260 {
                teammate.0.move_to(team_position_object.as_ref(), None);
            } else {
                if team_leader_go_direction.is_some() {
                    teammate.0.move_direction(team_leader_go_direction.unwrap());
                }
            }
        }
    }

    // 非队员移动逻辑
    let mates = select::select_creeps::select_my_mages_not_in_team(&team_map);
    if mates.is_some() {
        let mates = mates.as_ref().unwrap();
        for mate in mates {
            if get_ticks() < 260 && !is_close {
                mate.move_to(team_position_object.as_ref(), None);
            } else if get_ticks() >= 260 && !is_close {
                if enemy_creeps.is_some() {
                    let mate_enemy_creeps_closest =
                        find::find_closest_by_range(mate, &enemy_creeps);
                    if utils::get_range(
                        mate.unchecked_ref(),
                        mate_enemy_creeps_closest.as_ref().unwrap().unchecked_ref(),
                    ) >= 4
                        && utils::get_range(mate.unchecked_ref(), enemy_spawn.unchecked_ref()) > 1
                    {
                        let mate_mate_enemy_creeps_closest_path = find_path(
                            mate.unchecked_ref(),
                            mate_enemy_creeps_closest.as_ref().unwrap().unchecked_ref(),
                            None,
                        );
                        let mate_mate_enemy_creeps_closest_path_js_map = Map::new();
                        mate_mate_enemy_creeps_closest_path_js_map.set(
                            &JsValue::from("x"),
                            &JsValue::from(mate_mate_enemy_creeps_closest_path.path()[0].x),
                        );
                        mate_mate_enemy_creeps_closest_path_js_map.set(
                            &JsValue::from("y"),
                            &JsValue::from(mate_mate_enemy_creeps_closest_path.path()[0].y),
                        );
                        let mate_mate_enemy_creeps_closest_path_js_map_object =
                            Object::from_entries(
                                mate_mate_enemy_creeps_closest_path_js_map.as_ref(),
                            )
                            .unwrap();
                        mate.move_to(&mate_mate_enemy_creeps_closest_path_js_map_object, None);
                    } else if utils::get_range(
                        mate.unchecked_ref(),
                        mate_enemy_creeps_closest.as_ref().unwrap().unchecked_ref(),
                    ) == 3
                        && utils::get_range(mate.unchecked_ref(), enemy_spawn.unchecked_ref()) > 1
                    {
                        // 不动
                    } else if utils::get_range(
                        mate.unchecked_ref(),
                        mate_enemy_creeps_closest.as_ref().unwrap().unchecked_ref(),
                    ) < 3
                        && utils::get_range(mate.unchecked_ref(), enemy_spawn.unchecked_ref()) > 1
                    {
                        let mate_mate_enemy_creeps_closest_path = find_path(
                            mate.unchecked_ref(),
                            mate_enemy_creeps_closest.as_ref().unwrap().unchecked_ref(),
                            {
                                let fpo = FindPathOptions::new();
                                fpo.cost_matrix(&costs);
                                fpo.flee(true);
                                Some(fpo).as_ref()
                            },
                        );
                        let mate_mate_enemy_creeps_closest_path_js_map = Map::new();
                        mate_mate_enemy_creeps_closest_path_js_map.set(
                            &JsValue::from("x"),
                            &JsValue::from(mate_mate_enemy_creeps_closest_path.path()[0].x),
                        );
                        mate_mate_enemy_creeps_closest_path_js_map.set(
                            &JsValue::from("y"),
                            &JsValue::from(mate_mate_enemy_creeps_closest_path.path()[0].y),
                        );
                        let mate_mate_enemy_creeps_closest_path_js_map_object =
                            Object::from_entries(
                                mate_mate_enemy_creeps_closest_path_js_map.as_ref(),
                            )
                            .unwrap();
                        mate.move_to(&mate_mate_enemy_creeps_closest_path_js_map_object, None);
                    } else {
                        mate.move_to(enemy_spawn.as_ref(), None);
                    }
                } else {
                    mate.move_to(enemy_spawn.as_ref(), None);
                }
            } else if is_close {
                if enemy_creeps.is_some() {
                    let mate_enemy_creeps_closest =
                        find::find_closest_by_range(mate, &enemy_creeps);
                    if utils::get_range(
                        mate.unchecked_ref(),
                        my_spawn_enemy_creeps_closest
                            .as_ref()
                            .unwrap()
                            .unchecked_ref(),
                    ) >= 4
                        && utils::get_range(mate.unchecked_ref(), enemy_spawn.unchecked_ref()) > 1
                    {
                        mate.move_to(my_spawn_enemy_creeps_closest.as_ref().unwrap(), None);
                    } else if utils::get_range(
                        mate.unchecked_ref(),
                        my_spawn_enemy_creeps_closest
                            .as_ref()
                            .unwrap()
                            .unchecked_ref(),
                    ) == 3
                        && utils::get_range(mate.unchecked_ref(), enemy_spawn.unchecked_ref()) > 1
                    {
                        // 不动
                    } else if utils::get_range(
                        mate.unchecked_ref(),
                        my_spawn_enemy_creeps_closest
                            .as_ref()
                            .unwrap()
                            .unchecked_ref(),
                    ) < 3
                        && utils::get_range(mate.unchecked_ref(), enemy_spawn.unchecked_ref()) > 1
                    {
                        let mate_mate_enemy_creeps_closest_path = find_path(
                            mate.unchecked_ref(),
                            mate_enemy_creeps_closest.as_ref().unwrap().unchecked_ref(),
                            {
                                let fpo = FindPathOptions::new();
                                fpo.cost_matrix(&costs);
                                fpo.flee(true);
                                Some(fpo).as_ref()
                            },
                        );
                        let mate_mate_enemy_creeps_closest_path_js_map = Map::new();
                        mate_mate_enemy_creeps_closest_path_js_map.set(
                            &JsValue::from("x"),
                            &JsValue::from(mate_mate_enemy_creeps_closest_path.path()[0].x),
                        );
                        mate_mate_enemy_creeps_closest_path_js_map.set(
                            &JsValue::from("y"),
                            &JsValue::from(mate_mate_enemy_creeps_closest_path.path()[0].y),
                        );
                        let mate_mate_enemy_creeps_closest_path_js_map_object =
                            Object::from_entries(
                                mate_mate_enemy_creeps_closest_path_js_map.as_ref(),
                            )
                            .unwrap();
                        mate.move_to(&mate_mate_enemy_creeps_closest_path_js_map_object, None);
                    }
                }
            }
        }
    }

    // 战斗逻辑
    if mages.is_some() {
        let mages = mages.unwrap();
        for mage in &mages {
            if enemy_creeps.is_some() && (enemy_extensions.is_some() || enemy_towers.is_some()) {
                let mage_enemy_creeps_closest = find::find_closest_by_range(&mage, &enemy_creeps);
                let mage_enemy_creeps_in_3_range = find::find_in_x_range(&mage, &enemy_creeps, 3);
                let mage_enemy_creeps_in_3_range_lowest_hits =
                    find_lowest_hits_from_array(mage_enemy_creeps_in_3_range);
                let mage_enemy_towers_closest = find::find_closest_by_range(&mage, &enemy_towers);
                let mage_enemy_extensions_closest =
                    find::find_closest_by_range(&mage, &enemy_extensions);
                if utils::get_range(
                    mage.unchecked_ref(),
                    mage_enemy_creeps_closest.as_ref().unwrap().unchecked_ref(),
                ) <= 3
                    && utils::get_range(
                        mage.unchecked_ref(),
                        mage_enemy_creeps_closest.as_ref().unwrap().unchecked_ref(),
                    ) >= 2
                    && utils::get_range(mage.unchecked_ref(), enemy_spawn.unchecked_ref()) > 1
                {
                    mage.ranged_attack(mage_enemy_creeps_in_3_range_lowest_hits.as_ref().unwrap());

                    let mage_my_injured_creeps_in_1_range =
                        find::find_in_x_range(mage.as_ref(), &my_injured_creeps, 1);
                    if mage_my_injured_creeps_in_1_range.is_some() {
                        let mage_my_injured_creeps_in_1_range_lowest_hits =
                            find_lowest_hits_from_array(mage_my_injured_creeps_in_1_range).unwrap();
                        mage.heal(&mage_my_injured_creeps_in_1_range_lowest_hits);
                    } else {
                        mage.heal(mage);
                    }
                } else if utils::get_range(
                    mage.unchecked_ref(),
                    mage_enemy_creeps_closest.as_ref().unwrap().unchecked_ref(),
                ) <= 1
                    && utils::get_range(mage.unchecked_ref(), enemy_spawn.unchecked_ref()) > 1
                {
                    mage.ranged_mass_attack();

                    let mage_my_injured_creeps_in_1_range =
                        find::find_in_x_range(mage.as_ref(), &my_injured_creeps, 1);
                    if mage_my_injured_creeps_in_1_range.is_some() {
                        let mage_my_injured_creeps_in_1_range_lowest_hits =
                            find_lowest_hits_from_array(mage_my_injured_creeps_in_1_range).unwrap();
                        mage.heal(&mage_my_injured_creeps_in_1_range_lowest_hits);
                    } else {
                        mage.heal(mage);
                    }
                } else if enemy_towers.is_some() {
                    if utils::get_range(
                        mage.unchecked_ref(),
                        mage_enemy_towers_closest.as_ref().unwrap().unchecked_ref(),
                    ) <= 1
                    {
                        mage.ranged_mass_attack();
                    } else if utils::get_range(
                        mage.unchecked_ref(),
                        mage_enemy_towers_closest.as_ref().unwrap().unchecked_ref(),
                    ) <= 3
                    {
                        mage.ranged_attack(
                            mage_enemy_towers_closest
                                .as_ref()
                                .unwrap()
                                .unchecked_ref::<StructureTower>(),
                        );
                    }

                    let mage_my_injured_creeps_in_1_range =
                        find::find_in_x_range(mage.as_ref(), &my_injured_creeps, 1);
                    if mage_my_injured_creeps_in_1_range.is_some() {
                        let mage_my_injured_creeps_in_1_range_lowest_hits =
                            find_lowest_hits_from_array(mage_my_injured_creeps_in_1_range).unwrap();
                        mage.heal(&mage_my_injured_creeps_in_1_range_lowest_hits);
                    } else {
                        mage.heal(mage);
                    }
                } else if utils::get_range(mage.unchecked_ref(), enemy_spawn.unchecked_ref()) <= 3 {
                    if utils::get_range(mage.unchecked_ref(), enemy_spawn.unchecked_ref()) <= 1 {
                        mage.ranged_mass_attack();
                    } else if utils::get_range(mage.unchecked_ref(), enemy_spawn.unchecked_ref())
                        <= 3
                    {
                        mage.ranged_attack(enemy_spawn.unchecked_ref::<StructureSpawn>());
                    }

                    let mage_my_injured_creeps_in_1_range =
                        find::find_in_x_range(mage.as_ref(), &my_injured_creeps, 1);
                    if mage_my_injured_creeps_in_1_range.is_some() {
                        let mage_my_injured_creeps_in_1_range_lowest_hits =
                            find_lowest_hits_from_array(mage_my_injured_creeps_in_1_range).unwrap();
                        mage.heal(&mage_my_injured_creeps_in_1_range_lowest_hits);
                    } else {
                        mage.heal(mage);
                    }
                } else if enemy_extensions.is_some() {
                    if utils::get_range(
                        mage.unchecked_ref(),
                        mage_enemy_extensions_closest
                            .as_ref()
                            .unwrap()
                            .unchecked_ref(),
                    ) <= 1
                    {
                        mage.ranged_mass_attack();
                    } else if utils::get_range(
                        mage.unchecked_ref(),
                        mage_enemy_extensions_closest
                            .as_ref()
                            .unwrap()
                            .unchecked_ref(),
                    ) <= 3
                    {
                        mage.ranged_attack(
                            mage_enemy_extensions_closest
                                .as_ref()
                                .unwrap()
                                .unchecked_ref::<StructureExtension>(),
                        );
                    }

                    let mage_my_injured_creeps_in_1_range =
                        find::find_in_x_range(mage.as_ref(), &my_injured_creeps, 1);
                    if mage_my_injured_creeps_in_1_range.is_some() {
                        let mage_my_injured_creeps_in_1_range_lowest_hits =
                            find_lowest_hits_from_array(mage_my_injured_creeps_in_1_range).unwrap();
                        mage.heal(&mage_my_injured_creeps_in_1_range_lowest_hits);
                    } else {
                        mage.heal(mage);
                    }
                } else {
                    let mage_my_injured_creeps_in_1_range =
                        find::find_in_x_range(mage.as_ref(), &my_injured_creeps, 1);
                    let mage_my_injured_creeps_in_3_range =
                        find::find_in_x_range(mage.as_ref(), &my_injured_creeps, 3);
                    if mage_my_injured_creeps_in_1_range.is_some() {
                        let mage_my_injured_creeps_in_1_range_lowest_hits =
                            find_lowest_hits_from_array(mage_my_injured_creeps_in_1_range).unwrap();
                        mage.heal(&mage_my_injured_creeps_in_1_range_lowest_hits);
                    } else if mage_my_injured_creeps_in_3_range.is_some() {
                        let mage_my_injured_creeps_in_3_range_lowest_hits =
                            find_lowest_hits_from_array(mage_my_injured_creeps_in_3_range).unwrap();
                        mage.heal(&mage_my_injured_creeps_in_3_range_lowest_hits);
                    } else {
                        mage.heal(mage);
                    }
                }
            } else {
                if utils::get_range(mage.unchecked_ref(), enemy_spawn.unchecked_ref()) <= 3 {
                    if utils::get_range(mage.unchecked_ref(), enemy_spawn.unchecked_ref()) <= 1 {
                        mage.ranged_mass_attack();
                    }
                } else if utils::get_range(mage.unchecked_ref(), enemy_spawn.unchecked_ref()) <= 3 {
                    mage.ranged_attack(enemy_spawn.unchecked_ref::<StructureSpawn>());
                }

                let mage_my_injured_creeps_in_1_range =
                    find::find_in_x_range(mage.as_ref(), &my_injured_creeps, 1);
                if mage_my_injured_creeps_in_1_range.is_some() {
                    let mage_my_injured_creeps_in_1_range_lowest_hits =
                        find_lowest_hits_from_array(mage_my_injured_creeps_in_1_range).unwrap();
                    mage.heal(&mage_my_injured_creeps_in_1_range_lowest_hits);
                } else {
                    mage.heal(mage);
                }
            }
        }
    }
}
