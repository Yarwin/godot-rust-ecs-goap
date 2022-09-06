extends StaticBody2D
class_name StaticEntity

var entity_id
var game_node


func _ready() -> void:
	game_node = get_tree().get_root().get_node("Game")
	set_physics_process(false)

func on_death() -> void:
	print("I'm dying!")
	queue_free()
