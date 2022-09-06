use std::collections::HashMap;
use gdnative::prelude::*;
use gdnative::api::*;
use derive_godot_resource::ComponentGodotResource;
use crate::components::components_traits::GodotResourceComponent;

#[derive(Debug)]
pub struct GodotNode {
    // a representation of this entity in the godot game
    pub(crate) godot_entity: Ref<Node>
}

#[derive(Debug)]
pub struct Position {
    pub position: Vector2
}


#[derive(Debug, Default)]
#[derive(ComponentGodotResource)]
pub struct Thirst {
    #[expose_by_resource]
    pub per_second: f32,
    pub thirst: f32,
}

#[derive(Debug, Default)]
#[derive(ComponentGodotResource)]
pub struct Regeneration {
    #[expose_by_resource]
    pub amount: i32,
    #[expose_by_resource]
    pub every: f32,
    pub elapsed: f32
}

#[derive(Debug, Default)]
#[derive(ComponentGodotResource)]
pub struct Collectible {
    #[expose_by_resource]
    pub weight: i32,
    #[expose_by_resource]
    pub item_name: GodotString,
    pub picked: bool,
}


#[derive(Debug, Default)]
#[derive(ComponentGodotResource)]
pub struct Inventory {
    #[expose_by_resource]
    pub max_weight: i32,
    pub items: HashMap<GodotString, u32>
}

impl Inventory {
    pub fn add_one(&mut self, item_name: &GodotString) -> Result<(), ()> {
        return match self.items.contains_key(&item_name.to_lowercase()) {
            false => {
                self.items.insert(item_name.to_lowercase().clone(), 1);
                Ok(())
            }
            true => {
                *self.items.get_mut(&item_name.to_lowercase()).unwrap() += 1u32;
                Ok(())
            }
        }
    }

    pub fn remove_one(&mut self, item_name: &GodotString) -> Result<u32, ()> {
        return match self.items.contains_key(&item_name.to_lowercase()) {
            false => {
                Err(())
            }
            true => {
                let current_amount = *self.items.get(&item_name.to_lowercase()).unwrap();
                if current_amount == 1 {
                    self.items.remove(&item_name);
                    return Ok(0u32)
                }
                else if current_amount == 0 {
                    return Err(());
                }
                *self.items.get_mut(&item_name.to_lowercase()).unwrap() -= 1u32;
                Ok(current_amount - 1)
            }
        }

    }
    pub fn to_state(&self) -> HashMap<String, bool> {
        self.items.clone().into_iter().map(
            |(k, v)| {
                    if v != 0 {
                        return (format!("has_{}", k.to_string()), true);
                    }
                (format!("has_{}", k.to_string()), false)
            }
        ).collect()
    }
}


#[derive(ComponentGodotResource)]
#[derive(Debug, Default)]
pub struct Health{
    #[expose_by_resource]
    pub max: i32,
    #[from_field("max")]
    pub current: i32,
    pub suffer: i32,
}


#[derive(ComponentGodotResource)]
#[derive(Debug, Default)]
pub struct Speed {
    #[expose_by_resource]
    pub max: i32,
    #[expose_by_resource]
    pub acceleration: i32,
}


#[derive(ComponentGodotResource)]
#[derive(Debug, Default)]
pub struct Damage{
    #[expose_by_resource]
    pub damage: i32
}

