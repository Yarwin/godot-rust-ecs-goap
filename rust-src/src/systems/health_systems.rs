use gdnative::api::*;
use gdnative::prelude::*;
use hecs::{Entity, World};

use crate::components::agent_components::{Collectible, GodotNode, Health, Regeneration};


pub fn system_update_health(world: &mut World) {
    for (_id, hp) in world.query_mut::<&mut Health>() {
        hp.current -= hp.suffer;
        hp.suffer = 0;
    }
}

pub fn system_regeneration(world: &mut World, delta: f32) {
    for (_id, (hp, regen)) in world.query_mut::<(&mut Health, &mut Regeneration)>() {
        regen.elapsed += delta;
        if regen.elapsed >= regen.every {
            hp.suffer -= regen.amount;
            regen.elapsed = 0.0;
        }
    }
}

pub fn system_remove_dead(world: &mut World) {
    let mut to_remove: Vec<Entity> = Vec::new();
    for (id, (hp, node)) in world.query_mut::<(&mut Health, &mut GodotNode)>() {
        if hp.current <= 0 {
            to_remove.push(id);
            let node: TRef<Node2D> = unsafe { node.godot_entity.assume_safe() };
            if node.has_method("on_death") {
                // IMPORTANT - NODE SHOULD HANDLE ITS DELETION (for example by calling the queue_free) BY ITSELF
                unsafe { node.call(GodotString::from("on_death"), &[]) };
            }
        }
    }
    for entity in to_remove {
        world.despawn(entity).unwrap();
    }
}

pub fn system_remove_picked_items(world: &mut World) {
    let mut to_remove: Vec<Entity> = Vec::new();

    for (id, (item, node)) in world.query_mut::<(&mut Collectible, &mut GodotNode)>() {
        if item.picked {
            to_remove.push(id);
            let node: TRef<Node2D> = unsafe { node.godot_entity.assume_safe() };
            if node.has_method("on_death") {
                // IMPORTANT - NODE SHOULD HANDLE ITS DELETION (for example by calling the queue_free) BY ITSELF
                unsafe { node.call(GodotString::from("on_death"), &[]) };
            }
        }
    }
    for entity in to_remove {
        world.despawn(entity).unwrap();
    }
}
