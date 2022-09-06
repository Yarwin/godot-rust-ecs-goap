use gdnative::api::*;
use gdnative::prelude::*;
use hecs::EntityBuilder;

use crate::actions::actions_declaration::Actions;
use crate::actions::godot_action_resource::ActionResource;
use crate::components::agent_components::*;
use crate::components::components_traits::GodotResourceComponent;
use crate::components::godot_component_resources::EntityResource;
use crate::ecs::{Ecs, NewEntity};
use crate::goals::goals_declaration::Goals;
use crate::goals::godot_goal_resource::GoalResource;
use crate::goap::example_sensors::{FindObjectSensor, UpdatePositionSensor};
use crate::goap::goap_planner::{GAction, GGoal, GoapPlannerWorkingFacts};
use crate::goap_system::ecs_thinker::{GoapThinker, GoapWorkingMemoryFacts, ThinkerActionState};
use crate::goap_system::godot_blackboard::GoapBlackboardNode;

pub fn build_entity(ecs: &mut Ecs, new_entity_data: NewEntity) {
    let entity = ecs.world.reserve_entity();

    let mut builder = EntityBuilder::new();
    let mut action_builder = EntityBuilder::new();
    let mut goal_entities: Vec<GGoal<Goals>> = vec![];
    let mut action_entities: Vec<GAction<Actions>> = vec![];

    let entity_data = new_entity_data.entity_data;

    unsafe {entity_data.assume_safe() }.map(|data: &EntityResource, _ |{
        let entity_root_node = unsafe { data.root_node.as_ref().unwrap().assume_safe() }.instance(0).unwrap();
        unsafe { entity_root_node.assume_safe() }.set("position", new_entity_data.position.unwrap());
        unsafe { entity_root_node.assume_safe() }.set("entity_id", (entity.id() as i32).to_variant());
        builder.add(GodotNode { godot_entity: entity_root_node });

        let components = data.components.as_ref().unwrap();
        for some_variant in components.iter() {
            let some_resource: Ref<Resource> = some_variant.to_object::<Resource>().unwrap();
            let component_resource = String::from_variant(& unsafe { some_resource.assume_safe().call("get_component_type", &[])}).unwrap();
            match &component_resource[..] {
                "Health" => {builder.add(Health::from_resource(some_resource));},
                "Speed" => {builder.add(Speed::from_resource(some_resource));},
                "Damage" => {builder.add(Damage::from_resource(some_resource));},
                "Collectible" => {builder.add(Collectible::from_resource(some_resource));},
                "Inventory" => {builder.add(Inventory::from_resource(some_resource));},
                "Regeneration" => {builder.add(Regeneration::from_resource(some_resource));},
                _ => {panic!("Couldn't recognize the component!")}
            }
        }
        if data.is_ai_agent {
            let mut action_id: usize = 0;
            let mut goal_id: usize = 0;
            // create blackboard for agent
            let blackboard_node = Instance::emplace(GoapBlackboardNode::new()).into_shared();
            unsafe { entity_root_node.assume_safe() }.add_child(blackboard_node.clone(), true);
            unsafe { entity_root_node.assume_safe() }.set("blackboard", blackboard_node.to_variant());

            if data.actions.is_some() {
                for action_variant in data.actions.as_ref().unwrap().iter() {
                    let action: Ref<Resource> = action_variant.to_object::<Resource>().unwrap();
                    let action: Instance<ActionResource> = action.cast_instance::<ActionResource>().expect("Wrong resource type for entity!");
                    let goap_action = unsafe { action.assume_safe() }.map(
                        |action, _owner|
                            action.to_goap_action(action_id)
                    ).unwrap();
                    action_entities.push(goap_action);
                    action_id += 1;
                }
                for goal_variant in data.goals.as_ref().unwrap().iter() {
                    let goal: Ref<Resource> = goal_variant.to_object::<Resource>().unwrap();
                    let goal: Instance<GoalResource> = goal.cast_instance::<GoalResource>().expect("Wrong resource type for entity!");
                    let goap_goal = unsafe { goal.assume_safe() }.map (
                        |goal, _owner | goal.to_goap_goal(goal_id)
                    ).unwrap();
                    goal_entities.push(goap_goal);
                    goal_id += 1;
                }

                for sensor_variant in data.sensors.as_ref().unwrap().iter() {
                    let sensor_resource: Ref<Resource> = sensor_variant.to_object::<Resource>().unwrap();
                    let component_resource = String::from_variant(& unsafe { sensor_resource.assume_safe().call("get_component_type", &[])}).unwrap();
                    match &component_resource[..] { "FindObjectSensor" => {
                        action_builder.add(FindObjectSensor::from_resource(sensor_resource));
                    }
                        _ => {
                            panic!("Couldn't recognize the component named: {:#?}", component_resource)
                        }
                    }
                }

            }
            let initial_state = GoapPlannerWorkingFacts::default();

            action_builder.add(GoapThinker {
                actor_state: initial_state,
                working_memory: GoapWorkingMemoryFacts::default(),
                blackboard: blackboard_node.clone(),
                current_goal: None,
                current_plan: None,
                actions_available: action_entities,
                goals_available: goal_entities,
                state: ThinkerActionState::ShouldUpdate,
                owner: entity
            });
            action_builder.add(UpdatePositionSensor());
            ecs.thinkers.spawn_at(entity, action_builder.build());
        }

        // add a base node to the tree
        let owner = unsafe { new_entity_data.root_node.unwrap().assume_safe() };
        owner.add_child(entity_root_node, false);

    }).unwrap();


    if let Some(p) = new_entity_data.position {
        builder.add(Position { position: p.clone()});
    };
    ecs.world.insert(entity, builder.build()).unwrap();
}
