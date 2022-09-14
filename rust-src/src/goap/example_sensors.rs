use derive_godot_resource::ComponentGodotResource;
use crate::components::components_traits::GodotResourceComponent;
use gdnative::api::*;
use gdnative::prelude::*;
use hecs::{Entity, World};

use crate::components::agent_components::{GodotNode, Position};
use crate::goap_system::ecs_thinker::{Attribute, GoapThinker, GoapWorkingMemoryFact, ThinkerActionState};
use crate::goap_system::godot_blackboard::GodotEntityId;


pub trait Sensor {
    fn update(&mut self, world: &mut World, thinker: &mut GoapThinker, delta: f32);
}

pub struct UpdatePositionSensor();

impl Sensor for UpdatePositionSensor {
    fn update(&mut self, world: &mut World, thinker: &mut GoapThinker, _delta: f32) {
        let (agent, position) = world.query_one_mut::<(&GodotNode, &mut Position)>(thinker.owner).unwrap();
        let agent: TRef<Node2D> = unsafe { agent.godot_entity.assume_safe() };
        let result: Variant = unsafe {
            agent.call("get_position", &[])
        };
        let result = result.to::<Vector2>().unwrap();
        position.position = result;

    }
}

#[derive(Debug, Default)]
#[derive(ComponentGodotResource)]
pub struct FindObjectSensor {
    pub elapsed: f32,
    #[expose_by_resource]
    pub update_every: f32,
    #[expose_by_resource]
    pub object_names: StringArray
}

impl Sensor for FindObjectSensor {
    fn update(&mut self, world: &mut World, thinker: &mut GoapThinker, delta: f32) {
        if self.elapsed < self.update_every {
            self.elapsed += delta;
            return;
        }
        self.elapsed = 0.0;
        let agent = world.query_one_mut::<&mut GodotNode>(thinker.owner).unwrap();
        let group_names = self.object_names.to_vec();
        let godot_entity = agent.godot_entity;
        let owner_position = unsafe { agent.godot_entity.assume_safe() }.global_position();

        for object_name in group_names {
            let result: Variant = unsafe {
                godot_entity.assume_safe().call("get_colliding_entities_from_group", &[object_name.to_variant(), ])
            };
            let result = result.to::<Vec<(GodotEntityId, Vector2)>>().unwrap();

            if let Some(GoapWorkingMemoryFact::Objects(value)) = thinker.working_memory.get(&object_name.to_string()) {
                if value.len() != result.len() {
                    thinker.state = ThinkerActionState::ShouldUpdate;
                }
            }


            let result: Vec<Attribute<Entity>> = result.iter()
                .map(
                    |(entity, _position)| {
                        let mut entity_position = world.query_one::<&GodotNode>(entity.0).unwrap();
                        let entity_position = unsafe { entity_position.get().unwrap().godot_entity.assume_safe() }.global_position();
                        Attribute {
                            value: entity.0,
                            confidence: (1.0 / owner_position.distance_to(entity_position)),
                        }
                    }
                ).collect();
            if result.len() > 0 {
                thinker.actor_state.insert(
                    format!("see_{}", object_name.to_string().to_lowercase()),
                    true
                );
            }
            else {
                thinker.actor_state.insert(
                    format!("see_{}", object_name.to_string().to_lowercase()),
                    false
                );
            }
            thinker.working_memory.insert(
                object_name.to_string().to_lowercase(),
                GoapWorkingMemoryFact::Objects(
                    result
                )
            );

        }
    }

}

