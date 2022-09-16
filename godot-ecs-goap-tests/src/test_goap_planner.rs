use godot_rust_goap_ecs::goap::goap_planner;

pub(crate) fn run_tests() -> bool {
    let mut ok = true;

    // ok &= test_goap_planner();
    ok
}


struct TestAction {
    id: usize,
    name: String,
    preconditions: goap_planner::GoapPlannerWorkingFacts,
    post_conditions: goap_planner::GoapPlannerWorkingFacts
}


impl goap_planner::Action for TestAction {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_preconditions(&self) -> &goap_planner::GoapPlannerWorkingFacts {
        &self.preconditions
    }

    fn get_post_conditions(&self) -> &goap_planner::GoapPlannerWorkingFacts {
        &self.post_conditions
    }
}

crate::godot_itest! { test_goap_planner {
    // let's start with defining couple of actions:
    let actions = vec![
      TestAction {
            id: 1,
            name: "A to B".to_string(),
            preconditions: goap_planner::GoapPlannerWorkingFacts::default(),
            post_conditions: goap_planner::GoapPlannerWorkingFacts::from([("A".to_string(), true)])
        },
      TestAction {
            id: 2,
            name: "B to win".to_string(),
            preconditions: goap_planner::GoapPlannerWorkingFacts::from([("A".to_string(), true)]),
            post_conditions: goap_planner::GoapPlannerWorkingFacts::from([("Win".to_string(), true)])
        },
      TestAction {
            id: 3,
            name: "C to win".to_string(),
            preconditions: goap_planner::GoapPlannerWorkingFacts::default(),
            post_conditions: goap_planner::GoapPlannerWorkingFacts::from([("Win".to_string(), true)])
        },
    ];
    let initial_state = goap_planner::GoapPlannerWorkingFacts::default();
    let desired_state = goap_planner::GoapPlannerWorkingFacts::from([("Win".to_string(), true)]);
    // in real-world scenario we would arbitrary decide what is the action cost. In this very example, we are mocking the 
    let actions_refs_with_cost: Vec<(&TestAction, u32)> = actions
    .iter()
    .enumerate()
    .map(
        |(iteration_count, action)|
        (action, (iteration_count as u32 + 1).pow(2))
    ).collect();
    let plan = goap_planner::plan(&initial_state, &desired_state,
        &actions_refs_with_cost
    );


}
}