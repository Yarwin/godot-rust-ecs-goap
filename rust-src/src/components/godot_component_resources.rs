use gdnative::api::*;
use gdnative::prelude::*;


#[derive(NativeClass)]
#[inherit(Resource)]
#[derive(ToVariant, FromVariant)]
pub struct EntityResource {
    #[property]
    pub root_node: Option<Ref<PackedScene>>,
    #[property]
    pub components: Option<VariantArray>,
    #[property]
    pub is_ai_agent: bool,
    #[property]
    // in fact it's Vec<Instance<ActionResource>>
    pub actions: Option<VariantArray>,
    #[property]
    pub goals: Option<VariantArray>,
    #[property]
    pub ai_components: Option<VariantArray>
}

#[methods]
impl EntityResource {
    pub(crate) fn new(_owner: &Resource) -> Self {
        Self {
            root_node: None,
            components: None,
            is_ai_agent: false,
            actions: None,
            goals: None,
            ai_components: None
        }
    }
}
