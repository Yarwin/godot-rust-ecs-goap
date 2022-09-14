use gdnative::prelude::*;
use gdnative::api::*;
use crate::goals::goals_declaration::Goals;
use crate::goap::goap_planner::{GoapPlannerWorkingFacts};
use crate::goap_system::goal::GGoal;


#[derive(NativeClass)]
#[derive(ToVariant, FromVariant)]
#[derive(Debug)]
#[inherit(Resource)]
pub struct GoalResource {
    #[property]
    goal: Option<Goals>,
    #[property]
    priority: u32,
    #[property]
    name: String,
    #[property]
    desired_state: Option<Dictionary>,
}


#[methods]
impl GoalResource {

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

    pub fn to_goap_goal(&self, id: usize) -> GGoal<Goals> {
        let desired_state = self.option_dict_to_planner_facts(self.desired_state.as_ref());

        GGoal {
            id,
            name: self.name.clone(),
            priority: self.priority,
            desired_state,
            goal: self.goal.as_ref().unwrap().clone()
        }
    }

    fn new(_owner: &Resource) -> Self {
        Self {
            goal: None,
            priority: 0,
            name: "".to_string(),
            desired_state: None
        }
    }

}

