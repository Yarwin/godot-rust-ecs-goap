extends AnimationTree

var blackboard


func _ready() -> void:
	if owner.blackboard:
		blackboard = owner.blackboard
		
	

func _physics_process(_delta: float) -> void:
	yield(get_tree(), "idle_frame")
	set("parameters/Attacking/active", owner.blackboard.is_attacking and owner.blackboard.is_waiting)
