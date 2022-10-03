use hecs::World;
use crate::components::agent_components::Hunger;
use crate::goap_system::ecs_thinker::{Attribute, GoapThinker, GoapWorkingMemoryFact};


pub fn system_hunger(thinkers: &mut World, delta: f32) {
    for (_id, (thinker, hunger)) in thinkers.query_mut::<(&mut GoapThinker, &Hunger)>() {
        if let Some(GoapWorkingMemoryFact::Desire(hunger_attribute)) = thinker.working_memory.get_mut("hunger") {
            if hunger_attribute.value >= 100.0 {
                continue;
            }
            else if hunger_attribute.value <= 50.0 {
                thinker.actor_state.insert(
                    "is_hungry".to_string(),
                    false
                );
            }
            else if hunger_attribute.value > 50.0 {
                thinker.actor_state.insert(
                    "is_hungry".to_string(),
                    true
                );
            }
            hunger_attribute.value = (hunger_attribute.value + hunger.amount_per_second * delta).clamp(0.0, 100.0);
        } else {
            thinker.working_memory.insert(
                "hunger".to_string(),
                GoapWorkingMemoryFact::Desire(Attribute {value: hunger.amount_per_second * delta, confidence: 100.0 })
            );
        };

    }
}
