use crate::goap::goap_planner::GoapPlannerWorkingFacts;
use crate::goap_system::ecs_thinker::{GoapWorkingMemoryFact, GoapWorkingMemoryFacts};


pub fn is_valid(_current_memory: &GoapWorkingMemoryFacts, _current_facts: &GoapPlannerWorkingFacts) -> bool { true}


pub fn priority(original_priority: u32, current_memory: &GoapWorkingMemoryFacts) -> u32 {
    if let Some(GoapWorkingMemoryFact::Desire(hunger)) = current_memory.get("hunger") {
        if hunger.value > 50.0 {
            return original_priority;
        }
        else if hunger.value > 80.0 {
            return original_priority * 2;
        }
    }
    0
}
