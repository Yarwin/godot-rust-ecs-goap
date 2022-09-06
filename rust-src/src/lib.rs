extern crate core;

use gdnative::prelude::*;

#[macro_export]
macro_rules! get_node {
	($base:ident, $path:expr, $typ:ty) => {
		unsafe {
			$base
				.get_node($path)
				.unwrap()
				.cast::<$typ>()
				.assume_shared()
		}
	};
}

#[macro_export]
macro_rules! extract {
	($base:expr, $property:ident) => {
		unsafe { $base.assume_safe() }.map(|rust_instance, _owner| {rust_instance.$property.clone()}).unwrap()
	}
}

#[macro_export]
macro_rules! get {
	($base:expr, $property:ident) => {
		unsafe { $base.assume_safe() }.map(|rust_instance, _owner| {rust_instance.$property}).unwrap()
	}
}
#[macro_export]
macro_rules! get_ref {
	($base:expr, $property:ident) => {
		unsafe { $base.assume_safe() }.map(|rust_instance, _owner| {rust_instance.$property.as_ref()}).unwrap()
	}
}

#[macro_export]
macro_rules! set {
	($base:expr, $property:ident, $value: expr) => {
		unsafe { $base.assume_safe() }.map_mut(|rust_instance, _owner| {rust_instance.$property = $value}).unwrap()
	}
}

pub mod ecs;
pub mod godot_entity_builder;
pub mod components;
pub mod systems;
pub mod goap;

pub mod godot;
pub mod actions;
pub mod goap_system;
pub mod goals;


use godot::register_classes;

godot_init!(register_classes);
