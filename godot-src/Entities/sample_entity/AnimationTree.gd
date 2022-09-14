extends AnimationTree

var blackboard
var running = false

func _ready() -> void:
	if owner.blackboard:
		blackboard = owner.blackboard
		
	

func _physics_process(_delta: float) -> void:
	yield(get_tree(), "idle_frame")
	set("parameters/Attacking/active", owner.blackboard.is_attacking and owner.blackboard.is_waiting)
	set("parameters/Running/blend_amount", 1.0 if running else 0.0)
	running = false
