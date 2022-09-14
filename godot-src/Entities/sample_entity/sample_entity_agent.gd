extends KinematicBody2D

onready var vision := $Vision
onready var attack_area := $Attack
onready var physics_collision := $Vision/CollisionShape2D
onready var animation_player := $AnimationPlayer
onready var animation_tree := $AnimationTree
var game_node
var blackboard

var velocity = 60.0

class_name BasicEntity
var entity_id


func _ready() -> void:
	game_node = get_tree().get_root().get_node("Game")
	set_physics_process(false)


func move_to(target):
#	animation_player.play("run")
	var direction = position.direction_to(target)
	var _i = move_and_slide(velocity * direction)
	animation_tree.running = true



func reload_collisions():
	physics_collision.disabled = true
	physics_collision.disabled = false


func get_colliding_entities_from_group(group_name) -> Array:
	reload_collisions()
	var bodies_return = Array()
	var bodies = vision.get_overlapping_bodies()
	for body in bodies:
		if not body.is_in_group(group_name) or body == self or not body.entity_id:
			continue
		bodies_return.append([body.entity_id, body.position])
	return bodies_return


func attack():
	var bodies_return = Array()
	var bodies = attack_area.get_overlapping_bodies()
	for body in bodies:
		if body == self or not body.entity_id:
			continue
		game_node.ecs.add_input(
			{"EntityDamaged": {
				"entity": body.entity_id,
				"attacker": entity_id
				}})


func send_input_to_ecs(anim_name: String) -> void:
	game_node.ecs.add_input({"AnimationFinished": {"entity": entity_id, "animation_name": anim_name}})

