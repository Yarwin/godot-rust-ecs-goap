extends KinematicBody2D

var game_node


func _ready() -> void:
	game_node = get_tree().get_root().get_node("Game")
	set_physics_process(false)



func _on_Area2D_body_entered(body: Node) -> void:
	if body.is_in_group("can_be_scared") and ("entity_id" in body):
		game_node.ecs.add_input({"EntityScared": {"entity": body.entity_id}})
