use crate::consts;
use crate::game::spawns::{AllSpawnsMap, PositionWithoutMap};
use legion::systems::CommandBuffer;
use legion::World;
use rand::seq::SliceRandom;
use rustyhack_lib::ecs::components::{DisplayDetails, Inventory, MonsterDetails, Position, Stats};
use rustyhack_lib::ecs::monster::{AllMonsterDefinitions, Monster};
use rustyhack_lib::file_utils;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::process;
use uuid::Uuid;

pub(crate) fn initialise_all_monster_definitions() -> AllMonsterDefinitions {
    info!("About to initialise all monster definitions");
    let mut all_monster_definitions: AllMonsterDefinitions = HashMap::new();
    let mut file_location = file_utils::current_exe_location();
    file_location.pop();
    file_location.push(consts::ASSETS_DIRECTORY);
    file_location.push(consts::MONSTERS_DIRECTORY);
    let paths = file_utils::get_all_files_in_location(&file_location);
    for path in paths {
        let unwrapped_path = path.unwrap();
        let name = String::from(
            unwrapped_path
                .file_name()
                .to_str()
                .unwrap()
                .split('.')
                .next()
                .unwrap(),
        );
        let monster: Monster = get_monster_definition_from_path(&unwrapped_path.path());
        info!("Initialised monster: {:?}", &name);
        all_monster_definitions.insert(name, monster);
    }
    all_monster_definitions
}

fn get_monster_definition_from_path(path: &Path) -> Monster {
    let file = File::open(path).unwrap_or_else(|err| {
        error!(
            "Problem getting monster definition from file: {:?}, error: {}",
            path, err
        );
        process::exit(1);
    });
    let buf_reader = BufReader::new(file);
    serde_json::from_reader(buf_reader).unwrap_or_else(|err| {
        error!(
            "Problem deserializing monster definition from file: {:?}, error: {}",
            path, err
        );
        process::exit(1);
    })
}

pub(crate) fn spawn_initial_monsters(
    world: &mut World,
    all_monster_definitions: &AllMonsterDefinitions,
    all_spawns_map: &AllSpawnsMap,
) {
    info!("Spawning initial monsters.");
    let mut monsters_vec: Vec<(MonsterDetails, DisplayDetails, Position, Stats, Inventory)> =
        vec![];
    for (map, spawns) in all_spawns_map {
        for monster in &spawns.monsters {
            let mut current_monster = all_monster_definitions
                .get(&monster.monster_type)
                .unwrap_or_else(|| {
                    error!(
                        "Monster {} missing from all_monster_definitions.",
                        &monster.monster_type,
                    );
                    process::exit(1);
                })
                .clone();
            for spawn_position in &monster.spawn_positions {
                let position = Position {
                    update_available: false,
                    pos_x: spawn_position.x,
                    pos_y: spawn_position.y,
                    current_map: map.clone(),
                    velocity_x: 0,
                    velocity_y: 0,
                };
                current_monster.monster_details.id = Uuid::new_v4();
                current_monster.monster_details.spawn_position = position.clone();
                current_monster.position = position;
                info!(
                    "Spawned monster {} at position {:?}",
                    current_monster.monster_details.monster_type, current_monster.position
                );
                monsters_vec.push((
                    current_monster.monster_details.clone(),
                    current_monster.display_details,
                    current_monster.position,
                    current_monster.stats,
                    current_monster.inventory.clone(),
                ));
            }
        }
    }
    world.extend(monsters_vec);
}

pub(crate) fn spawn_single_monster(
    all_monster_definitions: &AllMonsterDefinitions,
    all_spawns_map: &AllSpawnsMap,
    map: &String,
    monster_type: &String,
    commands: &mut CommandBuffer,
) {
    info!("Spawning single monster.");
    let mut current_monster = all_monster_definitions
        .get(monster_type)
        .unwrap_or_else(|| {
            error!(
                "Monster {} missing from all_monster_definitions.",
                monster_type,
            );
            process::exit(1);
        })
        .clone();
    let all_spawn_positions = all_spawns_map.get(map).unwrap();
    let mut random_spawn_position: PositionWithoutMap;
    for monster_spawn_positions in &all_spawn_positions.monsters {
        if monster_spawn_positions.monster_type.eq(monster_type) {
            random_spawn_position = *monster_spawn_positions
                .spawn_positions
                .choose(&mut rand::thread_rng())
                .unwrap();

            let position = Position {
                update_available: false,
                pos_x: random_spawn_position.x,
                pos_y: random_spawn_position.y,
                current_map: map.clone(),
                velocity_x: 0,
                velocity_y: 0,
            };
            current_monster.monster_details.id = Uuid::new_v4();
            current_monster.monster_details.spawn_position = position.clone();
            current_monster.position = position.clone();
            commands.push((
                current_monster.monster_details.clone(),
                current_monster.display_details,
                current_monster.position,
                current_monster.stats,
                current_monster.inventory.clone(),
            ));
            info!(
                "Spawned {} at position: ({} {})",
                current_monster.monster_details.monster_type, position.pos_x, position.pos_y
            );
        }
    }
}
