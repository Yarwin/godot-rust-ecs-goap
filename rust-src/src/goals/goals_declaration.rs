use derive_godot_resource::GoapGoal;
use crate::goap::goap_planner::GoapGoal;
use gdnative::export::*;
use gdnative::export::Export;
use gdnative::export::hint::{EnumHint, IntHint};
use gdnative::prelude::*;

use crate::goap::goap_planner::{GoapPlannerWorkingFacts};
use crate::goap_system::ecs_thinker::GoapWorkingMemoryFacts;
use crate::goals;


#[derive(GoapGoal)]
#[derive(Debug, Clone)]
pub enum Goals {
    #[implementation="goals::default_goal"]
    DefaultGoal,
}


impl FromVariant for Goals {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> {
        let result = i64::from_variant(variant)?;
        match result {
            1 => Ok(Goals::DefaultGoal),
            _ => Err(FromVariantError::UnknownEnumVariant {
                variant: "i64".to_owned(),
                expected: &["None", "1"],
            }),
        }
    }
}



impl ToVariant for Goals {
    fn to_variant(&self) -> Variant {
        match self {
            Goals::DefaultGoal => {1.to_variant()},
        }
    }
}


impl Export for Goals {
    type Hint = IntHint<u32>;

    fn export_info(_hint: Option<Self::Hint>) -> ExportInfo {
        Self::Hint::Enum(EnumHint::new(vec![
            "None".to_owned(),
            "DefaultGoal".to_owned(),
        ]))
            .export_info()
    }
}
