#![allow(dead_code)]
extern crate core;

use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use heck::{ToSnakeCase};

macro_rules! resource {
    	($name:ty) => {
		Class {
			name: format!("{}Resource", stringify!($name).to_string().split("::").last().unwrap().to_string()),
			base: stringify!($name).to_string().split("::").last().unwrap().to_string(),
            full_path: format!("{}Resource", stringify!($name).to_string()),
            export_type: ExportType::Component,
		}
	};
    ($name:ty, $export_type:expr) => {
		Class {
			name: format!("{}Resource", stringify!($name).to_string().split("::").last().unwrap().to_string()),
			base: stringify!($name).to_string().split("::").last().unwrap().to_string(),
            full_path: format!("{}Resource", stringify!($name).to_string()),
            export_type: $export_type,
		}
	};
}

macro_rules! class {
	($name:ty) => {
		Class {
			name: stringify!($name).to_string().split("::").last().unwrap().to_string(),
			base: "Resource".to_string(),
            full_path: stringify!($name).to_string(),
            export_type: ExportType::GodotClass,
		}
	};
	($name:ty, $base:ident) => {
		Class {
			name: stringify!($name).to_string().split("::").last().unwrap().to_string(),
			base: stringify!($base).to_string(),
            full_path: stringify!($name).to_string(),
            export_type: ExportType::GodotClass,
		}
	};
    ($name:ident, $base:ident, $full_path: ident) => {
		Class {
			name: stringify!($name).to_string().split("::").last().unwrap().to_string(),
			base: stringify!($base).to_string(),
            full_path: stringify!($full_path).to_string(),
            export_type: ExportType::GodotClass,
		}
	};
}

fn main() {
    let cfg = NativeClasses {
        godot_gdns_dir: PathBuf::from("../godot-src/Native"),
        godot_gdnlib_res_path: PathBuf::from("res://godot-src/Native/Ecs.gdnlib"),
        rust_class_dir: PathBuf::from("src/godot"),
        classes: vec![
            // Nodes
            class!(ecs::EcsFactory, Reference),
            class!(ecs::Ecs, Node),
            class!(goap_system::godot_blackboard::GoapBlackboardNode, Node),
            class!(actions::godot_action_resource::ActionResource, Resource),
            class!(goals::godot_goal_resource::GoalResource, Resource),
            // Resources/data
            resource!(components::godot_component_resources::Entity),
            resource!(components::agent_components::Health),
            resource!(components::agent_components::Speed),
            resource!(components::agent_components::Damage),
            resource!(components::agent_components::Collectible),
            resource!(components::agent_components::Inventory),
            resource!(components::agent_components::Regeneration),
            resource!(goap::example_sensors::FindObjectSensor, ExportType::AIResource),
        ],
        mods: vec![
            "ecs".to_string(),
            "components".to_string(),
            "actions".to_string(),
            "goals".to_string(),
            "goap".to_string(),
            "goap_system".to_string(),
        ]
    };

    sync(cfg).expect("Sync configured correctly");
}

// ----------------------------------------------------------------------------------------------------------------------------------------------

fn sync(cfg: NativeClasses) -> Result<(), std::io::Error> {
    validate(&cfg)?;

    fs::create_dir_all(&cfg.godot_gdns_dir)?;
    fs::create_dir_all(&cfg.rust_class_dir)?;

    let mut camel_to_snake = HashMap::new();
    for class in cfg.classes.iter() {
        camel_to_snake.insert(class.name.as_str(), class.name.to_snake_case());
    }

    // Remove no longer needed .gdns native scripts
    for gdns_file in fs::read_dir(&cfg.godot_gdns_dir)? {
        let path = gdns_file?.path();
        let filename = path.file_name().unwrap().to_str().unwrap();

        if let Some(camel_name) = filename.strip_suffix(".gdns") {
            if !camel_to_snake.contains_key(&camel_name) {
                //panic!("remove {path:?}");
                fs::remove_file(path)?;
            }
        }
    }

    // Create new .gdns files
    for class in cfg.classes.iter() {
        let gdns_path = cfg.godot_gdns_dir.join(class.name.clone() + ".gdns");
        if !gdns_path.exists() {
            fs::write(gdns_path, make_gdns(&cfg, &class))?;
        }

    }

    let mod_path = cfg.rust_class_dir.join("mod.rs");

    // Below statement causes recompilation every time, since build.rs overwrites structs/mod.rs repeatedly
    //println!("cargo:rerun-if-changed={}", mod_path.display());

    fs::write(mod_path, make_rust_mod(&cfg.classes, &cfg.mods))?;

    Ok(())
}

fn make_gdns(cfg: &NativeClasses, class: &Class) -> String {
    format!(
        r#"[gd_resource type="NativeScript" load_steps=2 format=2]
[ext_resource path="{gdnlib}" type="GDNativeLibrary" id=1]
[resource]
resource_name = "{name}"
class_name = "{name}"
library = ExtResource( 1 )
script_class_name = "{name}"
"#,
        gdnlib = cfg.godot_gdnlib_res_path.display(),
        name = class.name
    )
}

fn make_rust_mod(classes: &[Class], mods_names: &Vec<String>) -> String {
    let mods_names = mods_names.join(", ");
    let mods = &format!("use crate::{{{}}};", mods_names);
    let mut registers = String::new();

    for class in classes.iter() {
        // let snake_name = class.name.to_snake_case();
        // uses += &format!("\npub use {}::*;", snake_name);
        registers += &format!("\n\thandle.add_class::<{}>();", class.full_path);
    }

    format!(
        r#"// Auto-generated; do not edit.
{mods}
mod init_hook;

pub fn register_classes(handle: gdnative::init::InitHandle) {{{registers}
    init_hook::init_panic_hook();
}}
"#,
        mods = mods,
        // uses = uses,
        registers = registers,
    )
}

fn validate(cfg: &NativeClasses) -> Result<(), std::io::Error> {
    if !cfg.godot_gdnlib_res_path.starts_with("res://") {
        error(".gdnlib path must be a Godot 'res://' path")
    } else {
        Ok(())
    }
}

fn error(message: &str) -> Result<(), std::io::Error> {
    Err(std::io::Error::new(std::io::ErrorKind::Other, message))
}

struct NativeClasses {
    pub godot_gdns_dir: PathBuf,
    pub godot_gdnlib_res_path: PathBuf,
    pub rust_class_dir: PathBuf,
    pub classes: Vec<Class>,
    pub mods: Vec<String>
}

enum ExportType {
    GodotClass,
    Component,
    AIResource,
}

struct Class {
    pub name: String,
    pub base: String,
    pub full_path: String,
    pub export_type: ExportType
}
