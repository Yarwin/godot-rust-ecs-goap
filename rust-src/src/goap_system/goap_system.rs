use gdnative::prelude::*;

use hecs::{Entity, World};
use crate::actions::actions_declaration::Actions;
use crate::components::agent_components::Inventory;
use crate::goap_system::action::GoapAction;
use crate::ecs::GlobalStateResource;
use crate::goals::goals_declaration::Goals;
use crate::goap::example_sensors::Sensor;
use crate::goap::goap_planner::{GoapPlannerWorkingFacts, plan};
use crate::goap_system::ecs_thinker::{GoapThinker, search_by_function, search_by_function_mut, ThinkerActionState};
use crate::goap_system::action::GAction;
use crate::goap_system::goal::GoapGoal;
use crate::goap_system::goal::GGoal;

pub fn system_update_dynamic_sensors<X>(thinkers: &mut World, world: &mut World, delta: f32)
    where X: 'static + Sensor + Send + Sync
{
    for (_e, (sensor, thinker)) in &mut thinkers.query::<(&mut X, &mut GoapThinker)>() {
        sensor.update(world, thinker, delta);
    }
}


fn goap_plan_system(world: &mut World, entity: Entity, thinker: &mut GoapThinker, global_facts: &GoapPlannerWorkingFacts) {
    let mut best_goal: Option<usize> = None;
    let mut highest_priority = 0u32;

    let mut initial_state = thinker.actor_state.clone();
    initial_state.extend(global_facts.into_iter().map(|(k, v)| (k.clone(), v.clone())));
    match world.query_one::<&Inventory>(entity) {
        Ok(mut inventory) => {
            match inventory.get() {
                Some(i) => {
                    initial_state.extend(i.to_state().into_iter().map(|(k, v)| (k.clone(), v.clone())));
                },
                None => {}
            }
        }
        Err(_) => {}
    }

    for goal_entity in &thinker.goals_available {
        if goal_entity.desired_state.iter().all(|(k, v)| ((initial_state.get(k) == Some(v)) || (!*v && (initial_state.get(k) == None)))) {
            continue
        }
        if !goal_entity.goal.is_valid(&thinker.working_memory, &initial_state) {
            continue
        }
        let priority = goal_entity.goal.priority(goal_entity.priority, &thinker.working_memory);
        if priority > highest_priority {
            best_goal = Some(goal_entity.id);
            highest_priority = priority;
        }
    }

    if let Some(g) = best_goal {
        thinker.state = ThinkerActionState::Executing;
        thinker.current_goal = Some(g);
        let goal: &GGoal<Goals> = search_by_function(&thinker.goals_available, |goal| goal.id == g).unwrap();
        let allowed_actions_with_cost: Vec<(&GAction<Actions>, u32)> = thinker.actions_available.iter().map(
            |action| (action, action.action.get_cost(action.cost, &initial_state))
        ).collect();
        let goap_plan = plan(&initial_state, &goal.desired_state, &allowed_actions_with_cost);
        set!(thinker.blackboard, current_goal, GodotString::from(goal.name.clone()));
        if let Some(plan) = goap_plan {
            thinker.current_plan = Some(plan.clone());
            for i in plan {
                let action: &GAction<Actions> = search_by_function(&thinker.actions_available, |action| action.id == i).unwrap();
                godot_print!("PLAN STEP: {:#?}", action.name);
            }
        }
    }
}

fn goap_execute_action_system(thinker: &mut GoapThinker, global_facts: &GoapPlannerWorkingFacts, global_state: &GlobalStateResource, owner: Entity, world: &mut World) {
    if let Some(plan) = thinker.current_plan.clone() {
        let action: &mut GAction<Actions> = search_by_function_mut(&mut thinker.actions_available, |action| action.id == plan[0]).unwrap();
        if action.action.perform(&mut thinker.working_memory, owner, world, global_state, &mut thinker.blackboard, global_facts) {
            thinker.current_plan.as_mut().unwrap().remove(0);
            if thinker.current_plan.as_ref().unwrap().len() == 0 {
                thinker.current_plan = None;
                thinker.state = ThinkerActionState::ShouldUpdate;
            }
        }
    }
    else {
        thinker.state = ThinkerActionState::ShouldUpdate;
    }
}

pub fn clear_blackboard(thinker: &mut GoapThinker) {
    if let Some(plan) = thinker.current_plan.clone() {
        let action: &mut GAction<Actions> = search_by_function_mut(&mut thinker.actions_available, |action| action.id == plan[0]).unwrap();
        action.action.clear(&mut thinker.blackboard);
    }
}


pub fn goap_system(thinkers: &mut World, world: &mut World, global_state: &GlobalStateResource, global_facts: &GoapPlannerWorkingFacts) {
    for (owner, thinker) in thinkers.query_mut::<&mut GoapThinker>() {
        match thinker.state {
            ThinkerActionState::ShouldUpdate => {
                clear_blackboard(thinker);
                goap_plan_system(world, owner, thinker, global_facts);
                goap_execute_action_system(thinker, global_facts, global_state, owner, world);
            }
            ThinkerActionState::Executing => {
                goap_execute_action_system(thinker, global_facts, global_state, owner, world);
            }
        }
    }
}
