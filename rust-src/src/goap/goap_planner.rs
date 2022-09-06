// Based on work of Samuel Loretan <tynril@gmail.com> – see: https://github.com/tynril/rgoap/blob/master/src/lib.rs

use std::collections::{HashMap};
use std::hash::{Hash, Hasher};
use gdnative::prelude::*;
use hecs::{Entity, World};

use pathfinding::prelude::astar;
use crate::ecs::{GlobalStateResource};
use crate::goap_system::ecs_thinker::GoapWorkingMemoryFacts;
use crate::goap_system::godot_blackboard::GoapBlackboardNode;

/// working memory facts used to formulate a working plan
pub type GoapPlannerWorkingFacts = HashMap<String, bool>;


pub trait GoapAction {
    fn is_valid(&self, current_state: &GoapPlannerWorkingFacts) -> bool;
    fn get_cost(&self, original_cost: u32, current_state: &GoapPlannerWorkingFacts) -> u32;
    fn perform(&mut self,
               working_memory: &mut GoapWorkingMemoryFacts,
               owner: Entity,
               world: &mut World,
               global_state: &GlobalStateResource,
               blackboard: &mut Instance<GoapBlackboardNode>,
               global_facts: &GoapPlannerWorkingFacts) -> bool;
    fn clear(&self, blackboard: &mut Instance<GoapBlackboardNode>);
}

pub trait GoapGoal {
    fn is_valid(&self, current_memory: &GoapWorkingMemoryFacts, current_facts: &GoapPlannerWorkingFacts) -> bool;
    fn priority(&self, original_priority: u32, current_memory: &GoapWorkingMemoryFacts) -> u32;
}

pub struct GGoal<T: GoapGoal + Sync + Send> {
    pub id: usize,
    pub name: String,
    pub priority: u32,
    pub desired_state: GoapPlannerWorkingFacts,
    pub goal: T
}

pub struct GAction<T: GoapAction + Sync + Send> {
    pub id: usize,
    pub name: String,
    pub cost: u32,
    pub action: T,
    pub pre_conditions: GoapPlannerWorkingFacts,
    pub post_conditions: GoapPlannerWorkingFacts,
}


#[derive(PartialEq, Eq, Clone)]
struct PlanNode {
    current_state: GoapPlannerWorkingFacts,
    action: Option<usize>,
}

impl Hash for PlanNode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        if let Some(action) = self.action {
            action.hash(state);
        }

        for (key, value) in &self.current_state {
            key.hash(state);
            value.hash(state);
        }
    }
}

impl PlanNode {
    fn initial(initial_state: &GoapPlannerWorkingFacts) -> Self {
        PlanNode {
            current_state: initial_state.clone(),
            action: None,
        }
    }

    fn child<T: GoapAction + Sync + Send>(parent_state: GoapPlannerWorkingFacts, action: &GAction<T>) -> PlanNode {
        let mut child = PlanNode {
            current_state: parent_state.clone(),
            action: Some(action.id)
        };

        for (name, value) in &action.post_conditions {
            child.current_state.insert(name.clone(), value.clone());
        }

        child
    }

    fn possible_next_nodes<T: GoapAction + Sync + Send>(&self, actions: &Vec<GAction<T>>, current_state: &GoapPlannerWorkingFacts) -> Vec<(PlanNode, u32)> {
        let mut nodes: Vec<(PlanNode, u32)> = vec![];
        for action in actions {
            if self.matches_target(&action.pre_conditions) {
                nodes.push((PlanNode::child(self.current_state.clone(), action), action.action.get_cost(action.cost, current_state)));
            }
        }
        nodes
    }

    /// Count the number of states in this node that aren't matching the given target.
    fn mismatch_count(&self, target: &GoapPlannerWorkingFacts) -> u32 {
        let mut count: u32 = 0;
        for (name, target_value) in target {
            if let Some(current_value) = self.current_state.get(name) {
                if current_value != target_value {
                    count += 1;
                }
            } else {
                count += 1;
            }
        }

        count
    }

    /// Returns `true` if the current node is a full match for the given target.
    fn matches_target(&self, target: &GoapPlannerWorkingFacts) -> bool {
        self.mismatch_count(target) == 0
    }
}


/// Formulates a plan to get from an initial state to a goal state using a set of allowed actions.
pub fn plan<T: GoapAction + Sync + Send>(initial_state: &GoapPlannerWorkingFacts,
            goal_state: &GoapPlannerWorkingFacts,
            allowed_actions: &Vec<GAction<T>>)
            -> Option<Vec<usize>> {
    // Builds our initial plan node.
    let start: PlanNode = PlanNode::initial(initial_state);

    // Runs our search over the states graph.
    if let Some((plan, _)) = astar(&start,
                                   |ref node: &PlanNode| node.possible_next_nodes(allowed_actions, initial_state),
                                   |ref node: &PlanNode| node.mismatch_count(goal_state),
                                   |ref node: &PlanNode| node.matches_target(goal_state)) {
        Some(plan.into_iter().skip(1).map(|ref node| node.action.unwrap()).collect())
    } else {
        None
    }
}
