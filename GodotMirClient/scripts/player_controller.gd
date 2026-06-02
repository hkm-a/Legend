extends CharacterBody2D

signal moved(world_position: Vector2, direction: int)

@export var move_speed := 180.0
@export var diagonal_y_scale := 0.58
@export var move_report_interval := 0.22

@onready var sprite: Sprite2D = $Sprite2D
@onready var label: Label = $NameLabel
@onready var health_fill: ColorRect = $HealthBack/HealthFill

var _move_report_elapsed := 0.0
var _last_reported_tile := Vector2i(-1, -1)
var _last_direction := 4

func _ready() -> void:
	label.text = "新手战士"

func set_character_name(character_name: String) -> void:
	var trimmed := character_name.strip_edges()
	if not trimmed.is_empty():
		label.text = trimmed

func set_hp(hp: int, max_hp: int) -> void:
	var ratio := 1.0 if max_hp <= 0 else clampf(float(hp) / float(max_hp), 0.0, 1.0)
	health_fill.size.x = 56.0 * ratio
	set_dead(hp <= 0)

func set_dead(is_dead: bool) -> void:
	modulate = Color(0.45, 0.45, 0.45, 0.72) if is_dead else Color.WHITE
	rotation_degrees = 80.0 if is_dead else 0.0

func _physics_process(delta: float) -> void:
	var input_vector := Input.get_vector("move_left", "move_right", "move_up", "move_down")
	var world_vector := Vector2(input_vector.x, input_vector.y * diagonal_y_scale)
	velocity = world_vector.normalized() * move_speed if world_vector.length() > 0.0 else Vector2.ZERO
	move_and_slide()

	if abs(input_vector.x) > 0.01:
		sprite.flip_h = input_vector.x < 0.0

	if input_vector.length() > 0.0:
		_last_direction = _input_to_direction(input_vector)
		_move_report_elapsed += delta
		var tile := _world_to_tile(position)
		if _move_report_elapsed >= move_report_interval or tile != _last_reported_tile:
			_move_report_elapsed = 0.0
			_last_reported_tile = tile
			moved.emit(position, _last_direction)

func _input_to_direction(input_vector: Vector2) -> int:
	if input_vector.y < -0.1 and abs(input_vector.x) <= 0.1:
		return 0
	if input_vector.y < -0.1 and input_vector.x > 0.1:
		return 1
	if abs(input_vector.y) <= 0.1 and input_vector.x > 0.1:
		return 2
	if input_vector.y > 0.1 and input_vector.x > 0.1:
		return 3
	if input_vector.y > 0.1 and abs(input_vector.x) <= 0.1:
		return 4
	if input_vector.y > 0.1 and input_vector.x < -0.1:
		return 5
	if abs(input_vector.y) <= 0.1 and input_vector.x < -0.1:
		return 6
	return 7

func _world_to_tile(world_position: Vector2) -> Vector2i:
	var map_x := (world_position.x / 48.0 + world_position.y / 24.0) * 0.5
	var map_y := (world_position.y / 24.0 - world_position.x / 48.0) * 0.5
	return Vector2i(roundi(map_x), roundi(map_y))
