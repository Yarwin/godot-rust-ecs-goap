// use hecs::{Entity, World};
// use crate::ecs::Ecs;
//
//
// #[derive(Clone)]
// pub enum ActionState {
//     // initial state, no action set
//     Init,
//     // action requested, should be performed ASAP and has it's status changed
//     Requested,
//     // action ongoing, should resume execution until it decides otherwise
//     Executing,
//     Cancelled,
//     Success,
//     Failure,
// }
//
//
// impl ActionState {
//     pub fn new() -> Self {
//         Self::default()
//     }
// }
//
//
// impl Default for ActionState {
//     fn default() -> Self {
//         Self::Init
//     }
// }
//
//
// pub trait ComponentAction {
//     fn get_weight(&self, world: &World, entity: Entity) -> f32;
//     fn entry(&self, world: &World, entity: Entity) -> ActionState;
//     fn perform(&mut self, world: &mut World, entity: Entity, delta: f32) -> ActionState;
//     fn exit(&mut self, world: &mut World, entity: Entity, delta: f32) -> ActionState;
// }
//
//
// pub struct ActionEntity(pub Box<dyn ComponentAction + Sync + Send>);
//
// #[derive(Clone)]
// pub struct Thinker {
//     pub state: ActionState,
//     pub best_action: Option<Entity>,
//     pub actions_available: Vec<Entity>,
//     pub owner: Entity
// }
//
//
// pub fn get_best_action(actions: &World, world: &World, thinker: &mut Thinker, _delta: f32) {
//     let mut best_score = 0.0;
//     let mut best_action = None;
//     for action_entity in &thinker.actions_available {
//
//         let score = actions.query_one::<&ActionEntity>(*action_entity).unwrap().get().unwrap().0.get_weight(world, thinker.owner);
//         if score > best_score {
//             best_score = score;
//             best_action = Some(action_entity);
//         }
//     }
//     if let Some(a) = best_action {
//         thinker.best_action = Some(*a);
//         thinker.state = ActionState::Requested;
//     }
// }
//
//
// pub fn reasoning_system(ecs: &mut Ecs, delta: f32) {
//     for (_e, thinker) in ecs.thinkers.query_mut::<&mut Thinker>() {
//         match thinker.state {
//             ActionState::Init => {
//                 get_best_action(&ecs.actions, &ecs.world, thinker, delta);
//             }
//             ActionState::Requested => {
//                 let mut action = ecs.actions.query_one_mut::<&mut ActionEntity>(thinker.best_action.unwrap()).unwrap();
//                 thinker.state = action.0.entry(&mut ecs.world, thinker.owner);
//             }
//             ActionState::Executing => {
//                 let mut action = ecs.actions.query_one_mut::<&mut ActionEntity>(thinker.best_action.unwrap()).unwrap();
//                 thinker.state = action.0.perform(&mut ecs.world, thinker.owner, delta);
//             }
//             ActionState::Cancelled => {
//
//             }
//             ActionState::Success => {
//                 let mut action = ecs.actions.query_one_mut::<&mut ActionEntity>(thinker.best_action.unwrap()).unwrap();
//                 action.0.exit(&mut ecs.world, thinker.owner, delta);
//                 thinker.state = ActionState::Init;
//             }
//             ActionState::Failure => {
//
//             }
//         }
//
//     }
//
// }
