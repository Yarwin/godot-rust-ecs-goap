use gdnative::prelude::*;
use gdnative::api::*;
use crate::ecs::{Ecs, EcsEvent};

pub fn system_send_events(ecs: &mut Ecs) {

    while !ecs.output_queue.is_empty() {
        let data = ecs.output_queue.pop_front().unwrap();
        match data {
            EcsEvent::CreateEntity(blueprint_name, root_node, position) => {
                let entity_blueprint: Ref<Resource> = (ecs.blueprints.get(&blueprint_name).unwrap()).clone();
                ecs.add_entity_to_queue(entity_blueprint, root_node, position);
            }
        }

    }
}
