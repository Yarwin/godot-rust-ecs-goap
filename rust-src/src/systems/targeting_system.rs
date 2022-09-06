use hecs::World;

use crate::components::agent_components::{GodotNode, Position};
use crate::goap_system::ecs_thinker::GoapThinker;

pub fn targeting_system(thinkers: &mut World, world: &mut World ) {
    for (entity, thinker) in thinkers.query_mut::<&mut GoapThinker>() {
        if let Some(target) = extract!(thinker.blackboard, target) {
            if get!(thinker.blackboard, is_waiting) {
                continue
            }
            let mut target_query = match world.query_one::<(&GodotNode, &Position)>(target.0) {
                Ok(target) => {
                    target
                }
                Err(_no_such_entity) => {
                    set!(thinker.blackboard, is_waiting, false);
                    set!(thinker.blackboard, is_attacking, false);
                    set!(thinker.blackboard, target, None);
                    return;
                }
            };
            let (_target_node, target_position) = target_query.get().unwrap().clone();

            let mut agent_query = world.query_one::<(&GodotNode, &Position)>(entity).unwrap();
            let (_self_node, self_position) = agent_query.get().unwrap().clone();

            if self_position.position.distance_to(target_position.position) < 20.0 {
                set!(thinker.blackboard, is_waiting, true);
                set!(thinker.blackboard, is_attacking, true);
            }
        }
    }
}
