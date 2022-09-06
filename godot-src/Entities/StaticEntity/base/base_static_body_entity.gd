extends Node2D
class_name Node2DEntity

var entity_id
var game_node


func _ready() -> void:
	game_node = get_tree().get_root().get_node("Game")
	set_physics_process(false)

func on_death() -> void:
#	spawn wood
	print("I'm dying!")
