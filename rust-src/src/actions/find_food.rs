use gdnative::prelude::*;

use hecs::{Entity, World};
use crate::ecs::GlobalStateResource;
use crate::goap::goap_planner::GoapPlannerWorkingFacts;
use crate::goap_system::ecs_thinker::GoapWorkingMemoryFacts;
use crate::goap_system::godot_blackboard::GoapBlackboardNode;

pub fn is_valid(_current_state: &GoapPlannerWorkingFacts) -> bool {
    true
}

pub fn get_cost(original_cost: u32, _current_state: &GoapPlannerWorkingFacts) -> u32 {
    original_cost
}


pub fn perform(_working_memory: &mut GoapWorkingMemoryFacts, _owner: Entity, _world: &mut World, _global_state: &GlobalStateResource, _blackboard: &mut Instance<GoapBlackboardNode>, _global_facts: &GoapPlannerWorkingFacts) -> bool {
    true
}

pub fn clear(_blackboard: &mut Instance<GoapBlackboardNode>) {
}
