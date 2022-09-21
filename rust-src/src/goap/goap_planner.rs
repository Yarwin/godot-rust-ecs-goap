// Based on work of Samuel Loretan <tynril@gmail.com> – see: https://github.com/tynril/rgoap/blob/master/src/lib.rs

use std::collections::{HashMap};
use std::hash::{Hash, Hasher};

use pathfinding::prelude::astar;

/// working memory facts used to formulate a working plan
pub type GoapPlannerWorkingFacts = HashMap<String, bool>;


pub trait Action {
    fn get_id(&self) -> usize;
    fn get_name(&self) -> String;
    fn get_preconditions(&self) -> &GoapPlannerWorkingFacts;
    fn get_post_conditions(&self) -> &GoapPlannerWorkingFacts;
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

    /// Makes a plan node from a parent state and an action applied to that state.
    fn child<T: Action>(parent_state: GoapPlannerWorkingFacts, action: &T) -> PlanNode {
        let mut child = PlanNode {
            current_state: parent_state.clone(),
            action: Some(action.get_id())
        };

        // Applies the post-condition of the action applied on our parent state.
        for (name, value) in action.get_post_conditions() {
            child.current_state.insert(name.clone(), value.clone());
        }

        child
    }

    /// Returns all possible nodes from this current state, along with the cost to get there.
    fn possible_next_nodes<T: Action>(&self, actions: &Vec<(&T, u32)>) -> Vec<(PlanNode, u32)> {
        let mut nodes: Vec<(PlanNode, u32)> = vec![];
        for (action, cost) in actions {
            if self.matches_target(action.get_preconditions()) {
                nodes.push((PlanNode::child(self.current_state.clone(), *action), *cost));
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
pub fn plan<T: Action>(initial_state: &GoapPlannerWorkingFacts,
                       goal_state: &GoapPlannerWorkingFacts,
                       allowed_actions_with_cost: &Vec<(&T, u32)>)
                       -> Option<Vec<usize>> {
    // Builds our initial plan node.
    let start: PlanNode = PlanNode::initial(initial_state);

    // Runs our search over the states graph.
    if let Some((plan, _)) = astar(&start,
                                   |ref node: &PlanNode| node.possible_next_nodes(allowed_actions_with_cost),
                                   |ref node: &PlanNode| node.mismatch_count(goal_state),
                                   |ref node: &PlanNode| node.matches_target(goal_state)) {
        Some(plan.into_iter().skip(1).map(|ref node| node.action.unwrap()).collect())
    } else {
        None
    }
}


#[cfg(test)]
mod tests {
    extern crate serde_json;
    use super::*;
    use serde_derive::*;
    use std::path::Path;
    use std::fs;

    #[derive(Deserialize)]
    struct TestAction {
        /// A simple implementation of our GOAP action
        id: usize,
        name: String,
        pre_conditions: GoapPlannerWorkingFacts,
        post_conditions: GoapPlannerWorkingFacts,
        cost: u32
    }

    impl Action for TestAction {
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

    #[derive(Deserialize)]
    struct TestCase {
        #[serde(skip_deserializing)]
        case_name: String,
        actions: Vec<TestAction>,
        initial_state: GoapPlannerWorkingFacts,
        goal_state: GoapPlannerWorkingFacts,
        expected_actions: Vec<u32>,
    }

    impl TestCase {
        /// Loads a test case from a JSON file.
        fn from_case_file(path: &Path) -> TestCase {
            let file = fs::File::open(path).unwrap();
            let mut case: TestCase = serde_json::from_reader(file).unwrap();
            case.case_name = String::from(path.file_name().unwrap().to_str().unwrap());
            case
        }

        /// Checks if the computed plan matches the expectation.
        fn assert_plan(&self) {
            let actions_refs_with_cost: Vec<(&TestAction, u32)> = self.actions
                    .iter()
                    .map(
                        |action|
                            (action, action.cost.clone())
                    ).collect();

            let plan = plan(&self.initial_state, &self.goal_state, &actions_refs_with_cost);

            if let Some(actions_list) = plan {
                if self.expected_actions != self.expected_actions {
                    panic!("{} failed: expected {:?}, got {:?}",
                           self.case_name,
                           self.expected_actions,
                    actions_list);
                }
            } else {
                if self.expected_actions.len() > 0 {
                    panic!("{} failed: expected {:?}, got no plan",
                           self.case_name,
                           self.expected_actions);
                }
            }
        }
    }

    #[test]
    fn run_test_files() {
        let paths = fs::read_dir("./test_data").unwrap();
        for path in paths {
            let case = TestCase::from_case_file(path.unwrap().path().as_path());
            case.assert_plan();
        }
    }

}

