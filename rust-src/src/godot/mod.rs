// Auto-generated; do not edit.
use crate::{ecs, components, actions, goals, goap, goap_system};
mod init_hook;

pub fn register_classes(handle: gdnative::init::InitHandle) {
	handle.add_class::<ecs::EcsFactory>();
	handle.add_class::<ecs::Ecs>();
	handle.add_class::<goap_system::godot_blackboard::GoapBlackboardNode>();
	handle.add_class::<actions::godot_action_resource::ActionResource>();
	handle.add_class::<goals::godot_goal_resource::GoalResource>();
	handle.add_class::<components::godot_component_resources::EntityResource>();
	handle.add_class::<components::agent_components::HealthResource>();
	handle.add_class::<components::agent_components::SpeedResource>();
	handle.add_class::<components::agent_components::DamageResource>();
	handle.add_class::<components::agent_components::CollectibleResource>();
	handle.add_class::<components::agent_components::InventoryResource>();
	handle.add_class::<components::agent_components::RegenerationResource>();
	handle.add_class::<goap::example_sensors::FindObjectSensorResource>();
    init_hook::init_panic_hook();
}
