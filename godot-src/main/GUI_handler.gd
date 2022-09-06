extends Node

var game_manager
onready var agent_entity_resource := preload("res://godot-src/Entities/sample_entity/components/sample_entity_agent.tres")

func _ready() -> void:
	game_manager = get_tree().get_root().get_node("Game")


func _on_Button_pressed() -> void:
	game_manager.ecs.add_entity(
		agent_entity_resource, 
		game_manager.entities, 
		game_manager.spawn_point.position)
