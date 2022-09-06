use crate::goap::goap_planner::GoapPlannerWorkingFacts;
use crate::goap_system::ecs_thinker::GoapWorkingMemoryFacts;

pub fn is_valid(_current_memory: &GoapWorkingMemoryFacts, _current_facts: &GoapPlannerWorkingFacts) -> bool {true}

pub fn priority(original_priority: u32, _current_memory: &GoapWorkingMemoryFacts) -> u32 {original_priority}
