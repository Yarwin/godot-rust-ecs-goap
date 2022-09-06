use gdnative::prelude::*;
use hecs::World;

use crate::components::agent_components::GodotNode;
use crate::goap_system::ecs_thinker::GoapThinker;


/// subsystem responsible for telling our agents how and where to move
pub fn navigation_system(thinkers: &mut World, world: &mut World ) {
    for (entity, thinker) in &mut thinkers.query::<&GoapThinker>() {
        if let Some(target) = extract!(thinker.blackboard, goto_target) {
            if get!(thinker.blackboard, is_waiting) {
                continue
            }
            let mut query = world.query_one::<&GodotNode>(entity).unwrap();
            let entity = query.get().unwrap();
            unsafe { entity.godot_entity.assume_safe().call("move_to", &[target.to_variant(), ])};
        }
    }
}
