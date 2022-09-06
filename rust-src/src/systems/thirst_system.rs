use hecs::World;
use crate::components::agent_components::Thirst;


pub fn thirst_system(world: &mut World, delta: f32) {
    for (_e, mut thirst) in world.query_mut::<&mut Thirst>() {
        thirst.thirst += thirst.per_second * delta;
        if thirst.thirst >= 100.0 {
            thirst.thirst = 100.0;
        }
    }
}
