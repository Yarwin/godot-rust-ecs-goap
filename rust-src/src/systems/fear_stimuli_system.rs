use hecs::World;
use crate::goap_system::ecs_thinker::{GoapThinker, GoapWorkingMemoryFact};
use crate::godot_print;

pub fn system_fear(thinkers: &mut World, delta: f32) {
    for (_id, thinker) in thinkers.query_mut::<&mut GoapThinker>() {
        if let Some(GoapWorkingMemoryFact::Desire(fear)) = thinker.working_memory.get_mut("fear") {
            if extract!(thinker.blackboard, is_covered) {
                fear.value = (fear.value - 20.0 * delta).clamp(0.0, 100.0);
            }

            if fear.value > 0.1 {
                thinker.actor_state.insert("is_scared".to_string(), true);
            } else {
                thinker.actor_state.insert("is_scared".to_string(), false);
            }
        }
    }
}
