use std::collections::{HashMap, VecDeque};

use gdnative::api::*;
use gdnative::prelude::*;
use hecs::{Entity, World};
use crate::components::agent_components::Health;

use crate::components::godot_component_resources::EntityResource;
use crate::goap::example_sensors::{FindObjectSensor, UpdatePositionSensor};
use crate::goap::goap_planner::GoapPlannerWorkingFacts;
use crate::goap_system::ecs_thinker::GoapThinker;
use crate::goap_system::goap_system::{goap_system, system_update_dynamic_sensors};
use crate::godot_entity_builder;
use crate::systems::godot_output_system::system_send_events;
use crate::systems::health_systems::{system_regeneration, system_remove_dead, system_remove_picked_items, system_update_health};
use crate::systems::inventory_system::{system_crafting, system_pickup_items};
use crate::systems::navigation_system::navigation_system;
use crate::systems::targeting_system::targeting_system;


#[derive(NativeClass)]
#[inherit(Reference)]
pub struct EcsFactory;

#[methods]
impl EcsFactory {
    fn new(_o: &Reference) -> Self {
        Self
    }

    #[export]
    fn new_ecs(&self, _o: &Reference) -> Instance<Ecs, Unique> {
        Ecs::new().emplace()
    }
}


#[derive(ToVariant, FromVariant)]
#[derive(Debug)]
pub enum GodotInput {
    AnimationFinished{ entity: u32, animation_name: GodotString },
    GameStateUpdate { name: GodotString, value: bool },
    EntityDamaged { entity: u32, attacker: u32 }
}


pub enum EcsEvent {
    /// outputs to be processed by Godot engine – in format Action(entity_id, *data)
    PlayAnimation(Entity, GodotString),
    MoveTo(Entity, Vector2),
    Rotate(Entity, f32),
    /// information to create some entity by "name" that is being specified in the ECS blueprint
    CreateEntity(GodotString, Option<Ref<Node>>, Option<Vector2>)
}

pub struct NewEntity {
    pub entity_data: Instance<EntityResource, Shared>,
    pub root_node: Option<Ref<Node>>,
    pub position: Option<Vector2>,
}

#[derive(ToVariant, FromVariant)]
pub enum GodotInputResource {
    Position { p: Vector2 },
    Entity { e: u32 },
    Fact { state: bool },
}

pub type GlobalStateResource = HashMap<GodotString, Vec<GodotInputResource>>;

#[derive(NativeClass)]
#[no_constructor]
#[inherit(Node)]
pub struct Ecs {
    // base world storing all the entities
    pub world: World,
    // a world for AI agents
    pub thinkers: World,
    // global "blackboard" with facts available for all the agents
    pub global_facts: GoapPlannerWorkingFacts,
    // a structure to keep information about global state available to all the thinkers
    pub resources: GlobalStateResource,
    #[property]
    pub root_node: Option<Ref<Node>>,
    pub blueprints: HashMap<GodotString, Ref<Resource>>,
    // new entities to spawn.
    new_entity_queue: VecDeque<NewEntity>,
    // Inputs – "events" – to be processed by the systems
    input_queue: VecDeque<GodotInput>,
    // outputs – an actions to be performed on Godot engine side
    pub output_queue: VecDeque<EcsEvent>,
}

#[methods]
impl Ecs {
    fn new() -> Self {
        Ecs {
            world: World::new(),
            thinkers: World::new(),
            global_facts: Default::default(),
            resources: Default::default(),
            root_node: None,
            blueprints: Default::default(),
            new_entity_queue: VecDeque::new(),
            input_queue: VecDeque::new(),
            output_queue: VecDeque::new(),
        }
    }

    #[export]
    fn add_resource(&mut self, _o: &Node, name: GodotString, resource: GodotInputResource) {
        if self.resources.contains_key(&name) {
            if let Some(resources) = self.resources.get_mut(&name) {
                resources.push(resource);
            }
        }
        else {
            self.resources.insert(name, vec![resource]);
        }
    }

    #[export]
    fn add_input(&mut self, _o: &Node, input: GodotInput) {
        self.input_queue.push_front(input);
    }

    #[export]
    fn add_blueprint(&mut self, _o: &Node, name: GodotString, blueprint: Ref<Resource>) {
        self.blueprints.insert(name, blueprint);
    }

    pub(crate) fn add_entity_to_queue(&mut self, entity_data: Ref<Resource>, root_node: Option<Ref<Node>>, position: Option<Vector2>) {
        let entity_resource: Instance<EntityResource> = entity_data.cast_instance::<EntityResource>().expect("Wrong resource type for entity!");
        self.new_entity_queue.push_back(NewEntity {
            entity_data: entity_resource,
            root_node: {if root_node.is_some() {root_node}
            else if self.root_node.is_some() {self.root_node.clone()}
            else { None
                // panic!("No root node to attach the node!") <- there can be entities that exists only in ECS - for example the items
            }},
            position
        });
    }

    #[export]
    fn add_entity(&mut self, _o: &Node, entity_data: Ref<Resource>, root_node: Option<Ref<Node>>, position: Option<Vector2>) {
        self.add_entity_to_queue(entity_data, root_node, position);
    }

    fn process_input_queue(&mut self) {
        while !self.input_queue.is_empty() {
            let input = self.input_queue.pop_front().unwrap();
            match input {
                GodotInput::AnimationFinished { animation_name: _a, entity: id } => {
                    let entity = unsafe { self.thinkers.find_entity_from_id(id) };
                    let thinker = self.thinkers.query_one_mut::<&mut GoapThinker>(entity).expect("NO SUCH ENTITY");
                    set!(thinker.blackboard, is_waiting, false);
                },
                GodotInput::EntityDamaged {entity, attacker: _attacker} => {
                    let entity = unsafe { self.world.find_entity_from_id(entity) };
                    let health = self.world.query_one_mut::<&mut Health>(entity);
                    match health {
                        Ok(health_component) => {
                            health_component.suffer += 1;
                        }
                        Err(_) => {}
                    }
                }
                GodotInput::GameStateUpdate {name, value} => {
                    self.global_facts.insert(name.to_string(), value);
                }
            }
        }
    }

    fn create_entities(&mut self) {
        while !self.new_entity_queue.is_empty() {
            let data = self.new_entity_queue.pop_front().unwrap();
            godot_entity_builder::build_entity(self, data);
        }
    }

    #[export]
    fn _physics_process(&mut self, _o: &Node, delta: f32) {
        // create new entities
        self.create_entities();

        // process inputs
        self.process_input_queue();

        // run "static" systems
        system_regeneration(&mut self.world, delta);

        // system_reset_collision_shapes - if we spawn new entity DIRECTLY inside the godot node area2D collision shape it won't be caught by the godot engine

        // run sensors and update agents memory
        system_update_dynamic_sensors::<UpdatePositionSensor>(&mut self.thinkers, &mut self.world, delta);
        system_update_dynamic_sensors::<FindObjectSensor>(&mut self.thinkers, &mut self.world, delta);

        // formulate plans && update agents blackboard
        goap_system(&mut self.thinkers, &mut self.world, &self.resources, &self.global_facts);

        // commit changes
        system_send_events( self);

        system_update_health(&mut self.world);
        // system responsible for picking up objects
        system_pickup_items(&mut self.world, &mut self.thinkers);

        // system responsible for moving agents around
        navigation_system(&mut self.thinkers, &mut self.world);
        // system responsible for rotating agents
        targeting_system(&mut self.thinkers, &mut self.world);
        // system responsible for agents attacks
        // weapon_system(&mut self.thinkers, &mut self.world);

        // crafting system
        system_crafting(&mut self.world, &mut self.thinkers, &mut self.output_queue);

        // !!DANGER!! - cleanup
        system_remove_dead(&mut self.world);
        system_remove_picked_items(&mut self.world);
    }
}
