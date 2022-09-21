use gdnative::prelude::*;
use hecs::{Entity, World};

use crate::components::agent_components::{Inventory, Position};
use crate::ecs::GlobalStateResource;
use crate::goap::goap_planner::GoapPlannerWorkingFacts;
use crate::goap_system::ecs_thinker::{get_most_desirable, get_least_desirable, GoapWorkingMemoryFact, GoapWorkingMemoryFacts};
use crate::goap_system::godot_blackboard::GoapBlackboardNode;
use crate::goap_system::godot_blackboard::GodotEntityId;

pub fn is_valid(_current_state: &GoapPlannerWorkingFacts) -> bool {
    true
}

pub fn get_cost(original_cost: u32, working_memory: &GoapWorkingMemoryFacts) -> u32 {
    if let Some(GoapWorkingMemoryFact::Objects(wood_pieces)) = working_memory.get("wood") {
        if let Some(wood_piece) = get_least_desirable(wood_pieces) {
            return (wood_piece.confidence / 3.5) as u32;
        };

    }
    return 100;
}

fn update_closest_wood_position(working_memory: &mut GoapWorkingMemoryFacts, world: &mut World, blackboard: &mut Instance<GoapBlackboardNode>) {
    if let Some(GoapWorkingMemoryFact::Objects(wood_pieces)) = working_memory.get(&String::from("wood")) {
            let closest_wood = if let Some(wood) = get_least_desirable(wood_pieces) {
                wood.value
            } else {
                return;
            };

        set!(blackboard, pickup_object, Some(GodotEntityId::from_entity(closest_wood)));
        let mut wood_pos = match world.query_one::<&Position>(closest_wood) {
            Ok(some_wood) => {
                some_wood
            }
            Err(_) => {
                return;
            }
        };
        let wood_pos = wood_pos.get().unwrap().position.clone();
        set!(blackboard, goto_target, Some(wood_pos));
    }
}


pub fn perform(working_memory: &mut GoapWorkingMemoryFacts, owner: Entity, world: &mut World, _global_state: &GlobalStateResource, blackboard: &mut Instance<GoapBlackboardNode>, _global_facts: &GoapPlannerWorkingFacts) -> bool {
    let inventory = world.query_one_mut::<&mut Inventory>(owner).unwrap();
    match inventory.items.get(&GodotString::from("wood")) {
        None => {
            match extract!(blackboard, pickup_object) {
                None => {
                    update_closest_wood_position(working_memory, world, blackboard);
                }
                Some(_o) => {}
            }
        }
        Some(i) => {
            if *i == 0 {
                update_closest_wood_position(working_memory, world, blackboard);
                return false;
            }
            return true;
        }
    }
    false
}

pub fn clear(blackboard: &mut Instance<GoapBlackboardNode>) {
    set!(blackboard, goto_target, None);
    set!(blackboard, pickup_object, None);
}
