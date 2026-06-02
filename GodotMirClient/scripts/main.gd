extends Node2D

const MirGateClient = preload("res://scripts/mir_gate_client.gd")
const PlayerScene = preload("res://scenes/player.tscn")

@onready var ground: Node2D = $World/Ground
@onready var status_label: Label = $UI/Panel/Margin/VBox/StatusLabel
@onready var host_edit: LineEdit = $UI/Panel/Margin/VBox/HostRow/HostEdit
@onready var port_spin: SpinBox = $UI/Panel/Margin/VBox/HostRow/PortSpin
@onready var account_edit: LineEdit = $UI/Panel/Margin/VBox/AccountRow/AccountEdit
@onready var password_edit: LineEdit = $UI/Panel/Margin/VBox/PasswordRow/PasswordEdit
@onready var server_edit: LineEdit = $UI/Panel/Margin/VBox/ServerRow/ServerEdit
@onready var character_edit: LineEdit = $UI/Panel/Margin/VBox/CharacterRow/CharacterEdit
@onready var job_spin: SpinBox = $UI/Panel/Margin/VBox/CreateOptionRow/JobSpin
@onready var sex_spin: SpinBox = $UI/Panel/Margin/VBox/CreateOptionRow/SexSpin
@onready var hair_spin: SpinBox = $UI/Panel/Margin/VBox/CreateOptionRow/HairSpin
@onready var query_button: Button = $UI/Panel/Margin/VBox/ActionRow/QueryButton
@onready var create_button: Button = $UI/Panel/Margin/VBox/ActionRow/CreateButton
@onready var enter_button: Button = $UI/Panel/Margin/VBox/ActionRow/EnterButton
@onready var message_edit: LineEdit = $UI/Panel/Margin/VBox/ChatRow/MessageEdit
@onready var say_button: Button = $UI/Panel/Margin/VBox/ChatRow/SayButton
@onready var attack_button: Button = $UI/Panel/Margin/VBox/ChatRow/AttackButton
@onready var revive_button: Button = $UI/Panel/Margin/VBox/ChatRow/ReviveButton
@onready var character_list: ItemList = $UI/Panel/Margin/VBox/CharacterList
@onready var log_text: TextEdit = $UI/Panel/Margin/VBox/LogText

var gate_client
var remote_players: Dictionary = {}

func _ready() -> void:
	build_demo_ground()
	setup_client()
	$World/Player.moved.connect(_on_player_moved)
	query_button.pressed.connect(_on_query_button_pressed)
	create_button.pressed.connect(_on_create_button_pressed)
	enter_button.pressed.connect(_on_enter_button_pressed)
	say_button.pressed.connect(_on_say_button_pressed)
	attack_button.pressed.connect(_on_attack_button_pressed)
	revive_button.pressed.connect(_on_revive_button_pressed)
	character_list.item_selected.connect(_on_character_selected)
	status_label.text = "输入账号后查询角色列表"
	say_button.disabled = true
	attack_button.disabled = true
	revive_button.disabled = true

func build_demo_ground() -> void:
	for y in range(-8, 9):
		for x in range(-12, 13):
			var tile := Polygon2D.new()
			tile.polygon = PackedVector2Array([
				Vector2(0, -24),
				Vector2(48, 0),
				Vector2(0, 24),
				Vector2(-48, 0),
			])
			tile.position = Vector2((x - y) * 48, (x + y) * 24)
			tile.color = Color(0.19 + (x + y) % 2 * 0.025, 0.24, 0.19, 1.0)
			ground.add_child(tile)

func setup_client() -> void:
	gate_client = MirGateClient.new()
	add_child(gate_client)
	gate_client.status_changed.connect(_on_status_changed)
	gate_client.log_added.connect(_on_log_added)
	gate_client.characters_loaded.connect(_on_characters_loaded)
	gate_client.game_entered.connect(_on_game_entered)
	gate_client.movement_confirmed.connect(_on_movement_confirmed)
	gate_client.player_joined.connect(_on_player_joined)
	gate_client.player_moved.connect(_on_player_moved_remote)
	gate_client.player_left.connect(_on_player_left)
	gate_client.attack_observed.connect(_on_attack_observed)
	gate_client.player_died.connect(_on_player_died)
	gate_client.player_revived.connect(_on_player_revived)
	gate_client.failed.connect(_on_query_failed)

func _on_query_button_pressed() -> void:
	var login := _read_login_form()
	if login.is_empty():
		return
	character_list.clear()
	say_button.disabled = true
	attack_button.disabled = true
	_set_actions_disabled(true)
	gate_client.query_characters(login["host"], login["port"], login["account"], login["password"], login["server"])

func _on_create_button_pressed() -> void:
	var login := _read_login_form()
	if login.is_empty():
		return
	var character_name := character_edit.text.strip_edges()
	if character_name.is_empty():
		status_label.text = "请输入要创建的角色名"
		return
	character_list.clear()
	say_button.disabled = true
	attack_button.disabled = true
	_set_actions_disabled(true)
	gate_client.create_character(login["host"], login["port"], login["account"], login["password"], character_name, login["server"], int(job_spin.value), int(sex_spin.value), int(hair_spin.value))

func _on_enter_button_pressed() -> void:
	var login := _read_login_form()
	if login.is_empty():
		return
	var character_name := character_edit.text.strip_edges()
	if character_name.is_empty():
		status_label.text = "请输入角色名，或先查询后点击角色"
		return
	say_button.disabled = true
	attack_button.disabled = true
	_set_actions_disabled(true)
	gate_client.enter_game(login["host"], login["port"], login["account"], login["password"], character_name, login["server"])

func _on_say_button_pressed() -> void:
	var message := message_edit.text.strip_edges()
	if message.is_empty():
		return
	gate_client.say(message)
	message_edit.clear()

func _on_attack_button_pressed() -> void:
	gate_client.attack(_first_remote_actor_id())
	_play_attack_flash($World/Player)

func _on_revive_button_pressed() -> void:
	gate_client.revive()

func _read_login_form() -> Dictionary:
	if gate_client.is_busy():
		return {}
	var host := host_edit.text.strip_edges()
	var account := account_edit.text.strip_edges()
	var password := password_edit.text
	if host.is_empty():
		status_label.text = "请输入 LoginGate 地址"
		return {}
	if account.is_empty() or password.is_empty():
		status_label.text = "请输入账号和密码"
		return {}
	return {
		"host": host,
		"port": int(port_spin.value),
		"account": account,
		"password": password,
		"server": server_edit.text,
	}

func _on_status_changed(text: String) -> void:
	status_label.text = text
	_on_log_added(text)

func _on_log_added(text: String) -> void:
	log_text.text += text + "\n"
	log_text.set_caret_line(log_text.get_line_count())

func _on_characters_loaded(characters: Array) -> void:
	_set_actions_disabled(false)
	character_list.clear()
	if characters.is_empty():
		character_list.add_item("角色列表为空")
		return
	for character in characters:
		var prefix := "* " if character.get("selected", false) else ""
		var name := str(character.get("name", ""))
		var text := "%s%s  Lv.%d  Job=%d  Sex=%d" % [
			prefix,
			name,
			character.get("level", 0),
			character.get("job", 0),
			character.get("sex", 0),
		]
		character_list.add_item(text)
		character_list.set_item_metadata(character_list.get_item_count() - 1, name)
		if character_edit.text.strip_edges().is_empty():
			character_edit.text = name

func _on_character_selected(index: int) -> void:
	var metadata = character_list.get_item_metadata(index)
	if metadata != null:
		character_edit.text = str(metadata)

func _on_game_entered(_actor_id: int, x: int, y: int, _dir_light: int, hp: int, max_hp: int) -> void:
	_set_actions_disabled(false)
	say_button.disabled = false
	attack_button.disabled = false
	revive_button.disabled = true
	$World/Player.position = _map_to_world(x, y)
	$World/Player.set_character_name(character_edit.text.strip_edges())
	$World/Player.set_hp(hp, max_hp)
	status_label.text = "进入游戏成功：X=%d Y=%d HP=%d/%d" % [x, y, hp, max_hp]

func _on_movement_confirmed(x: int, y: int, _direction: int) -> void:
	status_label.text = "坐标同步：X=%d Y=%d" % [x, y]

func _on_player_joined(actor_id: int, character_name: String, x: int, y: int, _direction: int, hp: int, max_hp: int, is_dead: bool) -> void:
	var player = _ensure_remote_player(actor_id, character_name)
	player.position = _map_to_world(x, y)
	player.set_hp(hp, max_hp)
	player.set_dead(is_dead)
	_on_log_added("玩家进入：%s X=%d Y=%d HP=%d/%d" % [character_name, x, y, hp, max_hp])

func _on_player_moved_remote(actor_id: int, character_name: String, x: int, y: int, _direction: int) -> void:
	var player = _ensure_remote_player(actor_id, character_name)
	player.position = _map_to_world(x, y)

func _on_player_left(actor_id: int, character_name: String) -> void:
	if not remote_players.has(actor_id):
		return
	remote_players[actor_id].queue_free()
	remote_players.erase(actor_id)
	_on_log_added("玩家离开：%s" % character_name)

func _on_attack_observed(actor_id: int, from_name: String, target_actor_id: int, target_name: String, damage: int, hp: int, max_hp: int, is_dead: bool) -> void:
	_on_log_added("攻击：%s -> %s 伤害=%d HP=%d/%d" % [from_name, target_name if not target_name.is_empty() else "空目标", damage, hp, max_hp])
	if remote_players.has(actor_id):
		_play_attack_flash(remote_players[actor_id])
	if target_actor_id == 0:
		return
	if remote_players.has(target_actor_id):
		remote_players[target_actor_id].set_hp(hp, max_hp)
		remote_players[target_actor_id].set_dead(is_dead)
		_play_hit_flash(remote_players[target_actor_id])
	elif target_name == character_edit.text.strip_edges():
		$World/Player.set_hp(hp, max_hp)
		$World/Player.set_dead(is_dead)
		revive_button.disabled = not is_dead
		attack_button.disabled = is_dead
		_play_hit_flash($World/Player)

func _on_player_died(actor_id: int, character_name: String, hp: int, max_hp: int) -> void:
	_on_log_added("死亡：%s" % character_name)
	if remote_players.has(actor_id):
		remote_players[actor_id].set_hp(hp, max_hp)
		remote_players[actor_id].set_dead(true)
	elif character_name == character_edit.text.strip_edges():
		$World/Player.set_hp(hp, max_hp)
		$World/Player.set_dead(true)
		attack_button.disabled = true
		revive_button.disabled = false
		status_label.text = "已死亡，点击复活"

func _on_player_revived(actor_id: int, character_name: String, x: int, y: int, _direction: int, hp: int, max_hp: int) -> void:
	_on_log_added("复活：%s HP=%d/%d" % [character_name, hp, max_hp])
	if remote_players.has(actor_id):
		remote_players[actor_id].position = _map_to_world(x, y)
		remote_players[actor_id].set_hp(hp, max_hp)
		remote_players[actor_id].set_dead(false)
	elif character_name == character_edit.text.strip_edges():
		$World/Player.position = _map_to_world(x, y)
		$World/Player.set_hp(hp, max_hp)
		$World/Player.set_dead(false)
		attack_button.disabled = false
		revive_button.disabled = true
		status_label.text = "复活成功：X=%d Y=%d HP=%d/%d" % [x, y, hp, max_hp]

func _on_query_failed(_message: String) -> void:
	_set_actions_disabled(false)
	say_button.disabled = not gate_client.is_in_game()
	attack_button.disabled = not gate_client.is_in_game()
	revive_button.disabled = true

func _on_player_moved(world_position: Vector2, direction: int) -> void:
	gate_client.walk_to_world(world_position, direction)

func _set_actions_disabled(disabled: bool) -> void:
	query_button.disabled = disabled
	create_button.disabled = disabled
	enter_button.disabled = disabled

func _ensure_remote_player(actor_id: int, character_name: String):
	if remote_players.has(actor_id):
		var existing = remote_players[actor_id]
		existing.set_character_name(character_name)
		return existing
	var player = PlayerScene.instantiate()
	player.name = "RemotePlayer%d" % actor_id
	player.set_process(false)
	player.set_physics_process(false)
	$World.add_child(player)
	player.set_character_name(character_name)
	player.get_node("Sprite2D/Body").color = Color(0.24, 0.5, 0.72, 1)
	player.get_node("Sprite2D/Cape").color = Color(0.08, 0.18, 0.38, 0.95)
	remote_players[actor_id] = player
	return player

func _first_remote_actor_id() -> int:
	for actor_id in remote_players.keys():
		return int(actor_id)
	return 0

func _play_attack_flash(player: Node) -> void:
	_flash_polygon(player.get_node("Sprite2D/SwordGlow"), Color(1, 0.95, 0.25, 1))

func _play_hit_flash(player: Node) -> void:
	_flash_polygon(player.get_node("Sprite2D/Body"), Color(1, 0.18, 0.12, 1))

func _flash_polygon(node: CanvasItem, color: Color) -> void:
	var original := node.modulate
	node.modulate = color
	var tween := create_tween()
	tween.tween_property(node, "modulate", original, 0.25)

func _map_to_world(x: int, y: int) -> Vector2:
	return Vector2((x - y) * 48, (x + y) * 24)
