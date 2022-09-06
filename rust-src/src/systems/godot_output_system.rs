use gdnative::prelude::*;
use gdnative::api::*;
use crate::components::agent_components::GodotNode;
use crate::ecs::{Ecs, EcsEvent};

pub fn system_send_events(ecs: &mut Ecs) {

    while !ecs.output_queue.is_empty() {
        let data = ecs.output_queue.pop_front().unwrap();
        match data {
            EcsEvent::PlayAnimation(entity, animation_name) => {
                let mut entity = ecs.world.query_one::<&GodotNode>(entity).unwrap();
                let entity = entity.get().unwrap();
                unsafe { entity.godot_entity.assume_safe().call("play_animation", &[animation_name.to_variant(), ])};
            }
            EcsEvent::MoveTo(entity, pos) => {
                let mut entity = ecs.world.query_one::<&GodotNode>(entity).unwrap();
                let entity = entity.get().unwrap();
                unsafe { entity.godot_entity.assume_safe().call("move_to", &[pos.to_variant(), ])};
            }
            EcsEvent::Rotate(_, _) => {}
            EcsEvent::CreateEntity(blueprint_name, root_node, position) => {
                let entity_blueprint: Ref<Resource> = (ecs.blueprints.get(&blueprint_name).unwrap()).clone();
                ecs.add_entity_to_queue(entity_blueprint, root_node, position);
            }
        }

    }
}
