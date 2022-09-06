extends StaticEntity

class_name TreeEntity

export (Resource) var wood_resource


func on_death() -> void:
#	spawn wood
	game_node.add_entity_builder(
		wood_resource,
		position
	)
	queue_free()
