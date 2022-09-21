tool
extends Node2D

onready var sprite := $Sprite

export (Resource) var entity_data
export (Texture) var editor_sprite

func _ready() -> void:
	if Engine.editor_hint and editor_sprite:
		sprite.texture = editor_sprite
	elif Engine.editor_hint and entity_data and entity_data.root_node:
		for variant in entity_data.root_node._bundled["variants"]:
			if variant is AtlasTexture:
				sprite.texture = variant
		
	elif not Engine.editor_hint:
		var game_manager := get_tree().get_root().get_node("Game")
		yield(get_tree(), "idle_frame")
		game_manager.ecs.add_entity(
			entity_data, 
			game_manager.entities, 
			position)
		queue_free()

