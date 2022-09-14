use derive_godot_resource::GoapAction;
use gdnative::export::Export;
use gdnative::export::hint::{EnumHint, IntHint};
use gdnative::prelude::*;
use gdnative::prelude::FromVariant;
use hecs::{Entity, World};

use crate::actions;
use crate::ecs::GlobalStateResource;
use crate::goap::goap_planner::GoapPlannerWorkingFacts;
use crate::goap_system::action::GoapAction;
use crate::goap_system::ecs_thinker::GoapWorkingMemoryFacts;
use crate::goap_system::godot_blackboard::GoapBlackboardNode;

#[derive(GoapAction)]
#[derive(Debug, Clone)]
pub enum Actions {
    #[implementation="actions::find_tree"]
    FindTree,
    #[implementation="actions::chop_tree"]
    ChopTree,
    #[implementation="actions::collect_wood"]
    CollectWood,
    #[implementation="actions::build_firepit"]
    BuildFirepit,
    #[implementation="actions::find_food"]
    FindFood,
    #[implementation="actions::find_cover"]
    FindCover,
    #[implementation="actions::calm_down"]
    CalmDown,
}


impl FromVariant for Actions {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> {
        let result = i64::from_variant(variant)?;
        match result {
            1 => Ok(Actions::FindTree),
            2 => Ok(Actions::ChopTree),
            3 => Ok(Actions::CollectWood),
            4 => Ok(Actions::BuildFirepit),
            5 => Ok(Actions::FindCover),
            6 => Ok(Actions::FindFood),
            7 => Ok(Actions::CalmDown),
            _ => Err(FromVariantError::UnknownEnumVariant {
                variant: "i64".to_owned(),
                expected: &["0", "1", "2", "3", "4", "5", "6"],
            }),
        }
    }
}

impl ToVariant for Actions {
    fn to_variant(&self) -> Variant {
        match self {
            Actions::FindTree => {1.to_variant()},
            Actions::ChopTree => {2.to_variant()},
            Actions::CollectWood => {3.to_variant()},
            Actions::BuildFirepit => {4.to_variant()},
            Actions::FindCover => {5.to_variant()},
            Actions::FindFood => {6.to_variant()},
            Actions::CalmDown => {7.to_variant()},
        }
    }
}

impl Export for Actions {
    type Hint = IntHint<u32>;

    fn export_info(_hint: Option<Self::Hint>) -> ExportInfo {
        Self::Hint::Enum(EnumHint::new(vec![
            "None".to_owned(),
            "FindTree".to_owned(),
            "ChopTree".to_owned(),
            "CollectWood".to_owned(),
            "BuildFirepit".to_owned(),
            "FindCover".to_owned(),
            "FindFood".to_owned(),
            "CalmDown".to_owned(),
        ]))
            .export_info()
    }
}
