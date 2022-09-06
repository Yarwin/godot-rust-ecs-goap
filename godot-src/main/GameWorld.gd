extends Node2D

onready var camera := $Camera2D
onready var tile_map := $TileMap


func _ready():
	set_camera_limits()


func set_camera_limits() -> void:
	var map_limits = tile_map.get_used_rect()
	var map_cellsize = tile_map.cell_size
	camera.limit_left = map_limits.position.x * map_cellsize.x
	camera.limit_right = map_limits.end.x * map_cellsize.x
	camera.limit_top = map_limits.position.y * map_cellsize.y
	camera.limit_bottom = map_limits.end.y * map_cellsize.y
