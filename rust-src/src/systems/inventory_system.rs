use std::collections::VecDeque;

use gdnative::prelude::*;
use hecs::World;

use crate::components::agent_components::{Collectible, GodotNode, Inventory};
use crate::ecs::EcsEvent;
use crate::goap_system::ecs_thinker::{GoapThinker, GoapWorkingMemoryFact};

pub fn system_pickup_items(world: &mut World, thinkers: &mut World) {
    for (entity, thinker) in thinkers.query_mut::<&mut GoapThinker>() {
        if let Some(pickup) = extract!(thinker.blackboard, pickup_object) {
            let item_query = world.query_one_mut::<(&Collectible, &GodotNode)>(pickup.0);

            let (item_name, _weight, item_pos) = match item_query {
                Ok((i, node)) => {
                    (i.item_name.clone(), i.weight.clone(), unsafe { node.godot_entity.assume_safe() }.position())
                }
                Err(_) => {
                    // send event info - pickup failed
                    continue
                }
            };

            let (agent_node, inventory) = world.query_one_mut::<(&GodotNode, &mut Inventory)>(entity).unwrap();
            if unsafe { agent_node.godot_entity.assume_safe() }.position().distance_to(item_pos) > 25.0
            {
                return;
            }

            inventory.add_one(&item_name).expect("failed to add an item to the inventory!");
            let item = world.query_one_mut::<&mut Collectible>(pickup.0).unwrap();
            item.picked = true;
            set!(thinker.blackboard, pickup_object, None);
            set!(thinker.blackboard, goto_target, None);
        }
    }
}

pub fn system_crafting(world: &mut World, thinkers: &mut World, event_queue: &mut VecDeque<EcsEvent>) {
    for (entity, thinker) in thinkers.query_mut::<&mut GoapThinker>() {
        if let Some(craft_target) = extract!(thinker.blackboard, craft_target) {
            match craft_target.as_str() {
                "firepit" => {
                    let inventory = world.query_one_mut::<&mut Inventory>(entity).unwrap();
                    match inventory.items.get_mut(&GodotString::from("wood")) {
                        None => {}
                        Some(i) => {
                            if *i == 0 {
                                continue
                            }
                            *i -= 1;
                            event_queue.push_front(EcsEvent::CreateEntity(
                                GodotString::from("firepit"),
                                None,
                                extract!(thinker.blackboard, interact_position)))
                        }
                    }

                }
                "eat_food" => {
                    let inventory = world.query_one_mut::<&mut Inventory>(entity).unwrap();
                    match inventory.items.get_mut(&GodotString::from("food")) {
                        None => {}
                        Some(i) => {
                            if *i == 0 {
                                continue
                            }
                            *i -= 1;
                            if let Some(GoapWorkingMemoryFact::Desire(hunger_attribute)) = thinker.working_memory.get_mut("hunger") {
                                hunger_attribute.value = 0.0;
                            }
                        }
                    }
                }
                _ => {}
            }
        }

    }
}