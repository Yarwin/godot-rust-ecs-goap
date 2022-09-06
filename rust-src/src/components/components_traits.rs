use gdnative::prelude::*;
use gdnative::api::*;

pub trait GodotResourceComponent {
    fn from_resource(resource: Ref<Resource>) -> Self;
}
