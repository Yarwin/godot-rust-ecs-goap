extends Node

var ecs

onready var entities := get_node("%Entities")
onready var spawn_point := get_node("%Spawnpoint")

var ecs_handle_entity := preload("res://godot-src/Entities/EditorEntity/EditorEntity.tscn")
var firepit_blueprint := preload("res://godot-src/Entities/blueprints/firepit_resource.tres")


###############################################################################
# Builtin functions                                                           #
###############################################################################

func _ready() -> void:
	var ecs_factory = EcsFactory.new()
	ecs = ecs_factory.new_ecs()
	ecs.root_node = self
	add_child(ecs)
	ecs.add_blueprint("firepit", firepit_blueprint)


func _input(event: InputEvent) -> void:
	if event is InputEventKey:
		if event.scancode == KEY_SPACE and event.pressed:
			var _i = get_tree().reload_current_scene()

###############################################################################
# Private functions                                                           #
###############################################################################

###############################################################################
# Public functions                                                            #
###############################################################################


func add_2D_entity(some_entity: Node2D):
	entities.add_child(some_entity)


func add_entity_builder(enity_data: Resource, position: Vector2):
	var entity_builder = ecs_handle_entity.instance()
	entity_builder.position = position
	entity_builder.entity_data = enity_data
	entities.add_child(entity_builder)


###############################################################################
# Connections                                                                 #
###############################################################################
