use gdnative::prelude::*;

use hecs::{Entity, World};
use crate::components::agent_components::{GodotNode, Inventory};
use crate::ecs::GlobalStateResource;
use crate::goap::goap_planner::GoapPlannerWorkingFacts;
use crate::goap_system::ecs_thinker::{get_least_desirable, GoapWorkingMemoryFact, GoapWorkingMemoryFacts};
use crate::goap_system::godot_blackboard::GoapBlackboardNode;
use crate::goap_system::godot_blackboard::GodotEntityId;

pub fn is_valid(_current_state: &GoapPlannerWorkingFacts) -> bool {
    true
}

pub fn get_cost(original_cost: u32, _working_memory: &GoapWorkingMemoryFacts) -> u32 {
    original_cost
}

fn update_closest_mushroom_position(working_memory: &mut GoapWorkingMemoryFacts, world: &mut World, blackboard: &mut Instance<GoapBlackboardNode>) {
    if let Some(GoapWorkingMemoryFact::Objects(mushrooms)) = working_memory.get(&String::from("food")) {
        let closest_mushroom = if let Some(mushroom) = get_least_desirable(mushrooms) {
            mushroom.value
        } else {
            return;
        };
        let mut mushroom_node = match world.query_one::<&GodotNode>(closest_mushroom) {
            Ok(some_mushroom) => {
                some_mushroom
            }
            Err(_) => {
                return;
            }
        };
        let mushroom_pos = unsafe { mushroom_node.get().unwrap().godot_entity.assume_safe() }.global_position();
        set!(blackboard, pickup_object, Some(GodotEntityId::from_entity(closest_mushroom)));
        set!(blackboard, goto_target, Some(mushroom_pos));
    }
}


pub fn perform(working_memory: &mut GoapWorkingMemoryFacts, owner: Entity, world: &mut World, _global_state: &GlobalStateResource, blackboard: &mut Instance<GoapBlackboardNode>, _global_facts: &GoapPlannerWorkingFacts) -> bool {
    let inventory = world.query_one_mut::<&mut Inventory>(owner).unwrap();
    match inventory.items.get(&GodotString::from("food")) {
        None => {
            match extract!(blackboard, pickup_object) {
                None => {
                    update_closest_mushroom_position(working_memory, world, blackboard);
                }
                Some(_o) => {}
            }
        }
        Some(i) => {
            if *i == 0 {
                update_closest_mushroom_position(working_memory, world, blackboard);
                return false;
            }
            set!(blackboard, craft_target, Some(String::from("eat_food")));
            return true;
        }
    }
    false
}

pub fn clear(blackboard: &mut Instance<GoapBlackboardNode>) {
    set!(blackboard, goto_target, None);
    set!(blackboard, pickup_object, None);
    set!(blackboard, craft_target, None);
}
