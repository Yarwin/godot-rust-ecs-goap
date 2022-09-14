use crate::goap::goap_planner::GoapPlannerWorkingFacts;
use crate::goap_system::ecs_thinker::GoapWorkingMemoryFacts;

pub struct GGoal<T: GoapGoal + Sync + Send> {
    pub id: usize,
    pub name: String,
    pub priority: u32,
    pub desired_state: GoapPlannerWorkingFacts,
    pub goal: T
}


pub trait GoapGoal {
    fn is_valid(&self, current_memory: &GoapWorkingMemoryFacts, current_facts: &GoapPlannerWorkingFacts) -> bool;
    fn priority(&self, original_priority: u32, current_memory: &GoapWorkingMemoryFacts) -> u32;
}
