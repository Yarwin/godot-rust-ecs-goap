use gdnative::export::Export;
use gdnative::export::hint::IntHint;
use gdnative::prelude::*;
use hecs::Entity;

#[derive(Clone, Debug)]
pub struct GodotEntityId(pub(crate) Entity);

impl GodotEntityId {
    pub fn from_entity(entity: Entity) -> Self {
        GodotEntityId(entity)
    }
}

impl ToVariant for GodotEntityId {
    fn to_variant(&self) -> Variant {
        (u64::from(self.0.id())).to_variant()
    }
}

impl FromVariant for GodotEntityId {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> {
        Ok(GodotEntityId(Entity::from_bits(variant.to::<u64>().expect("Wrong variant")).expect("wrong variant")))
    }
}

impl Export for GodotEntityId {
    type Hint = IntHint<u64>;

    fn export_info(hint: Option<Self::Hint>) -> ExportInfo {
        hint.map_or_else(
            || ExportInfo::new(VariantType::I64),
            Self::Hint::export_info,
        )
    }
}


#[derive(NativeClass)]
#[derive(ToVariant, FromVariant)]
#[no_constructor]
#[inherit(Node)]
/// Node that contains agent's blackboard
pub struct GoapBlackboardNode {
    #[property]
    pub goto_target: Option<Vector2>,
    #[property]
    pub target: Option<GodotEntityId>,
    #[property]
    pub pickup_object: Option<GodotEntityId>,
    #[property]
    pub craft_target: Option<String>,
    #[property]
    pub interact_position: Option<Vector2>,
    #[property]
    pub is_waiting: bool,
    #[property]
    pub is_attacking: bool,
    #[property]
    pub current_goal: GodotString
}

#[methods]
impl GoapBlackboardNode {
    pub fn new() -> Self {
        GoapBlackboardNode {
            goto_target: None,
            target: None,
            pickup_object: None,
            craft_target: None,
            interact_position: None,
            is_waiting: false,
            is_attacking: false,
            current_goal: Default::default()
        }
    }

    pub fn clear(&mut self) {
        self.goto_target = None;
        self.target = None;
        self.pickup_object = None;
        self.craft_target = None;
        self.interact_position = None;
        self.is_waiting = false;
        self.is_attacking = false;
    }
}
