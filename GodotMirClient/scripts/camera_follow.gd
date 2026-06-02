extends Camera2D

@export var target_path: NodePath
@export var follow_smoothing := 8.0

@onready var target: Node2D = get_node(target_path)

func _ready() -> void:
	position = target.position

func _process(delta: float) -> void:
	position = position.lerp(target.position, 1.0 - exp(-follow_smoothing * delta))
