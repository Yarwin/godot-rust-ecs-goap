use gdnative::prelude::*;

use hecs::{Entity, World};
use crate::components::agent_components::Position;
use crate::ecs::{GlobalStateResource, GodotInputResource};
use crate::goap::goap_planner::GoapPlannerWorkingFacts;
use crate::goap_system::ecs_thinker::GoapWorkingMemoryFacts;
use crate::goap_system::godot_blackboard::GoapBlackboardNode;

pub fn is_valid(_current_state: &GoapPlannerWorkingFacts) -> bool {
    true
}

pub fn get_cost(original_cost: u32, _current_state: &GoapPlannerWorkingFacts) -> u32 {
    original_cost
}


pub fn perform(_working_memory: &mut GoapWorkingMemoryFacts, owner: Entity, world: &mut World, global_state: &GlobalStateResource, blackboard: &mut Instance<GoapBlackboardNode>, global_facts: &GoapPlannerWorkingFacts) -> bool {
    match global_facts.get("is_firepit_burning") {
        None => {}
        Some(is_firepit_burning) => {
            if *is_firepit_burning {
                clear(blackboard);
                return true;
            }
        }
    }
    let firepit_positions: &Vec<GodotInputResource> = global_state.get(&GodotString::from("FirePitPosition")).unwrap();
    let agent_position = world.query_one_mut::<&Position>(owner).unwrap();
    if let GodotInputResource::Position { p } = firepit_positions[0] {
        if let Some(target) = extract!(blackboard, goto_target) {
            if target.distance_to(agent_position.position) < 20.0 {
                if !get!(blackboard, is_waiting) {
                    set!(blackboard, craft_target, Some(String::from("firepit")));
                    set!(blackboard, interact_position, Some(target));
                    set!(blackboard, goto_target, None);
                }
            }
        } else {
            set!(blackboard, goto_target, Some(p));
        }
    }
    false
}

pub fn clear(blackboard: &mut Instance<GoapBlackboardNode>) {
    set!(blackboard, goto_target, None);
    set!(blackboard, interact_position, None);
    set!(blackboard, craft_target, None);
}
