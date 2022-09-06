extends Position2D

var game_node

func _ready() -> void:
	yield(get_tree(), "idle_frame")
	game_node = get_tree().get_root().get_node("Game")
	game_node.ecs.add_resource("FirePitPosition",
		{"Position": {"p": position}})
	queue_free()
