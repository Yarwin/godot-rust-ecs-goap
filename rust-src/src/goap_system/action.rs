use gdnative::prelude::*;
use hecs::{Entity, World};
use crate::ecs::GlobalStateResource;
use crate::goap::goap_planner::{Action, GoapPlannerWorkingFacts};
use crate::goap_system::ecs_thinker::GoapWorkingMemoryFacts;
use crate::goap_system::godot_blackboard::GoapBlackboardNode;


pub enum ActionResultEnum {
    Continue,
    ContinueAndUpdateSensors,
    Finished,
    FinishAndUpdateSensors
}


pub struct GAction<T: GoapAction + Sync + Send> {
    pub id: usize,
    pub name: String,
    pub cost: u32,
    pub action: T,
    pub pre_conditions: GoapPlannerWorkingFacts,
    pub post_conditions: GoapPlannerWorkingFacts,
}


pub trait GoapAction {
    fn is_valid(&self, current_state: &GoapPlannerWorkingFacts) -> bool;
    fn get_cost(&self, original_cost: u32, working_memory: &GoapWorkingMemoryFacts) -> u32;
    fn perform(&mut self,
               working_memory: &mut GoapWorkingMemoryFacts,
               owner: Entity,
               world: &mut World,
               global_state: &GlobalStateResource,
               blackboard: &mut Instance<GoapBlackboardNode>,
               global_facts: &GoapPlannerWorkingFacts) -> bool;
    fn clear(&self, blackboard: &mut Instance<GoapBlackboardNode>);
}

impl<T: GoapAction + Sync + Send> Action for GAction<T> {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_preconditions(&self) -> &GoapPlannerWorkingFacts {
        &self.pre_conditions
    }

    fn get_post_conditions(&self) -> &GoapPlannerWorkingFacts {
        &self.post_conditions
    }
}