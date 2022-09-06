extends Camera2D

export (float) var camera_speed = 500.0
onready var camera_zoom = get_zoom()


func _process(delta: float) -> void:
	var movement_dir = Vector2(
		Input.get_action_strength("move_right") - Input.get_action_strength("move_left"),
		Input.get_action_strength("move_back") - Input.get_action_strength("move_forward")
	)
	position.x = clamp(position.x + (movement_dir.x * delta * camera_speed), limit_left, limit_right)
	position.y = clamp(position.y + (movement_dir.y * delta * camera_speed), limit_top, limit_bottom)

