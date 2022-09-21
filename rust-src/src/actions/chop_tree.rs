use gdnative::prelude::*;
use hecs::{Entity, World};
use crate::components::agent_components::Position;
use crate::ecs::GlobalStateResource;
use crate::goap::goap_planner::GoapPlannerWorkingFacts;
use crate::goap_system::ecs_thinker::{get_most_desirable, get_least_desirable, GoapWorkingMemoryFact, GoapWorkingMemoryFacts};
use crate::goap_system::godot_blackboard::{GoapBlackboardNode, GodotEntityId};


pub fn is_valid(_current_state: &GoapPlannerWorkingFacts) -> bool {
    true
}

pub fn get_cost(original_cost: u32, working_memory: &GoapWorkingMemoryFacts) -> u32 {
    if let Some(GoapWorkingMemoryFact::Objects(trees)) = working_memory.get("tree") {
        if let Some(tree) = get_least_desirable(trees) {
            return (tree.confidence / 7.0) as u32;
        };

    }
    return 100;
}

pub fn update_closest_tree_position(working_memory: &mut GoapWorkingMemoryFacts, world: &mut World, blackboard: &mut Instance<GoapBlackboardNode>) {
    if let Some(GoapWorkingMemoryFact::Objects(trees)) = working_memory.get("tree") {
        let closest_tree = if let Some(tree) = get_least_desirable(trees) {
            tree.value
        } else {
            return;
        };

        let tree_pos = world.query_one::<&Position>(closest_tree);
        match tree_pos {
            Ok(mut tree_pos) => {
                let tree_pos = tree_pos.get().unwrap().position.clone();
                set!(blackboard, target, Some(GodotEntityId::from_entity(closest_tree)));
                set!(blackboard, goto_target, Some(tree_pos));
            }
            Err(_) => {}
        }
    }
}


pub fn perform(working_memory: &mut GoapWorkingMemoryFacts, _owner: Entity, world: &mut World, _global_state: &GlobalStateResource, blackboard: &mut Instance<GoapBlackboardNode>, _global_facts: &GoapPlannerWorkingFacts) -> bool {
    return if let Some(target) = extract!(blackboard, target) {
        // check if target - the tree - exists. In our naive implementation chopped down tree should be just despawned from the World
        if !world.contains(target.0) {
            clear(blackboard);
            return true;
        }
        false
    } else {
        // get a tree
        update_closest_tree_position(working_memory, world, blackboard);
        false
    }
}

pub fn clear(blackboard: &mut Instance<GoapBlackboardNode>) {
    set!(blackboard, target, None);
    set!(blackboard, goto_target, None);
    set!(blackboard, is_waiting, false);
    set!(blackboard, is_attacking, false);
}
