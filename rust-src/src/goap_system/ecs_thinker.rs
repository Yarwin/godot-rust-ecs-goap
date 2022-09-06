use std::collections::HashMap;
use hecs::Entity;
use crate::goap::goap_planner::{GAction, GGoal, GoapPlannerWorkingFacts};
use crate::{Instance, Vector2};
use crate::actions::actions_declaration::Actions;
use crate::goals::goals_declaration::Goals;
use crate::goap_system::godot_blackboard::GoapBlackboardNode;

pub enum ThinkerActionState {
    /// Thinker will recalculate the current goal ASAP. By default Thinker should recalculate goals every time when the world state change
    ShouldUpdate,
    /// Thinker has some plan and will execute it ASAP
    Executing,
}

#[derive(Debug)]
pub struct Attribute<T> {
    pub(crate) value: T,
    pub(crate) confidence: f32,
}

pub type GoapWorkingMemoryFacts = HashMap<String, GoapWorkingMemoryFact>;

#[derive(Debug)]
pub enum GoapWorkingMemoryFact {
    Positions(Vec<Attribute<Vector2>>),
    Objects(Vec<Attribute<Entity>>),
    Desire(Attribute<u32>)
}

pub fn get_most_desirable<T>(attributes: &Vec<Attribute<T>>) -> Option<&Attribute<T>> {
    if attributes.len() == 0 {
        return None
    }
    let most_desirable: &Attribute<T> = attributes
        .iter()
        .fold(
            None::<&Attribute<T>>, |a, b|  {
                return match a {
                    Some(attribute) => {
                        if attribute.confidence > b.confidence {
                            return Some(attribute)
                        }
                        Some(b)
                    }
                    None => Some(b)
                };
            }
        ).unwrap();
    return Some(most_desirable)
}

pub struct GoapThinker {
    pub actor_state: GoapPlannerWorkingFacts,
    pub working_memory: GoapWorkingMemoryFacts,
    pub blackboard: Instance<GoapBlackboardNode>,
    pub current_goal: Option<usize>,
    pub current_plan: Option<Vec<usize>>,
    pub actions_available: Vec<GAction<Actions>>,
    pub goals_available: Vec<GGoal<Goals>>,
    pub state: ThinkerActionState,
    pub owner: Entity
}

pub fn search_by_function<T> (objects: &Vec<T>, condition: impl Fn(&T) -> bool) -> Option<&T> {
    for object in objects {
        if condition(object) {
            return Some(object);
        }
    }
    None
}

pub fn search_by_function_mut<T> (objects: &mut Vec<T>, condition: impl Fn(&mut T) -> bool) -> Option<&mut T> {
    for object in objects {
        if condition(object) {
            return Some(object);
        }
    }
    None
}
