use gdnative::prelude::*;
use gdnative::api::*;
use crate::actions::actions_declaration::Actions;
use crate::goap::goap_planner::{GAction, GoapPlannerWorkingFacts};

#[derive(NativeClass)]
#[derive(ToVariant, FromVariant)]
#[derive(Debug)]
#[inherit(Resource)]
pub struct ActionResource {
    #[property]
    action: Option<Actions>,
    #[property]
    cost: u32,
    #[property]
    name: String,
    #[property]
    pre_conditions: Option<Dictionary>,
    #[property]
    post_conditions: Option<Dictionary>,
}

#[methods]
impl ActionResource {

    pub fn option_dict_to_planner_facts(&self, dict: Option<&Dictionary>) -> GoapPlannerWorkingFacts {
        if let Some(pre_conditions) = dict {
            return if !pre_conditions.is_empty() {
                pre_conditions.iter().map(
                    |(k, v)|
                        (k.to::<GodotString>().unwrap().to_string(), v.to::<bool>().unwrap())
                ).collect::<GoapPlannerWorkingFacts>()
            } else {
                GoapPlannerWorkingFacts::default()
            }
        }
        return GoapPlannerWorkingFacts::default();
    }

    pub fn to_goap_action(&self, id: usize) -> GAction<Actions> {
        let pre_conditions = self.option_dict_to_planner_facts(self.pre_conditions.as_ref());
        let post_conditions = self.option_dict_to_planner_facts(self.post_conditions.as_ref());

        GAction {
            id,
            name: self.name.clone(),
            cost: self.cost,
            action: self.action.as_ref().unwrap().clone(),
            pre_conditions,
            post_conditions
        }
    }

    fn new(_owner: &Resource) -> Self {
        Self {
            action: None,
            cost: 0,
            name: "".to_string(),
            pre_conditions: Default::default(),
            post_conditions: Default::default()
        }
    }

}

