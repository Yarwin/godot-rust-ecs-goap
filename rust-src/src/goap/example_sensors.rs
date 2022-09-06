use derive_godot_resource::ComponentGodotResource;
use crate::components::components_traits::GodotResourceComponent;
use gdnative::api::*;
use gdnative::prelude::*;
use hecs::{Entity, World};

use crate::components::agent_components::{GodotNode, Position};
use crate::goap_system::ecs_thinker::{Attribute, GoapThinker, GoapWorkingMemoryFact, ThinkerActionState};


fn manhattan_dist(pos0: Vector2, pos1: Vector2) -> f32 {
    let dx = (pos0.x - pos1.x).abs();
    let dy = (pos0.y - pos1.y).abs();
    dx + dy
}

pub trait Sensor {
    fn update(&mut self, world: &mut World, thinker: &mut GoapThinker, delta: f32);
}

pub struct UpdatePositionSensor();

impl Sensor for UpdatePositionSensor {
    fn update(&mut self, world: &mut World, thinker: &mut GoapThinker, _delta: f32) {
        let (agent, position) = world.query_one_mut::<(&GodotNode, &mut Position)>(thinker.owner).unwrap();
        let agent: TRef<Node> = unsafe { agent.godot_entity.assume_safe() };
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
        let (agent, owner_position) = world.query_one_mut::<(&mut GodotNode, &Position)>(thinker.owner).unwrap();
        let owner_position = owner_position.position.clone();
        let vec_of_strings = self.object_names.to_vec();
        let godot_entity = agent.godot_entity.clone();

        for object_name in vec_of_strings {
            let result: Variant = unsafe {
                godot_entity.assume_safe().call("get_colliding_entities_from_group", &[object_name.to_variant(), ])
            };
            let result = result.to::<Vec<(u32, Vector2)>>().unwrap();

            if let Some(GoapWorkingMemoryFact::Objects(value)) = thinker.working_memory.get(&object_name.to_string()) {
                if value.len() == result.len() {
                    continue;
                }
            }


            let result: Vec<Attribute<Entity>> = result.iter()
                .map(
                    |v| {
                        let entity = unsafe { world.find_entity_from_id(v.0) }.clone();
                        let mut entity_position = world.query_one::<&Position>(entity).unwrap();
                        let entity_position = entity_position.get().unwrap();
                        Attribute {
                            value: entity,
                            confidence: (1.0 / manhattan_dist(
                                owner_position,
                                entity_position.position,
                            ))
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
            thinker.state = ThinkerActionState::ShouldUpdate;
        }
    }

}

