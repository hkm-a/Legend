extends Node

signal status_changed(text: String)
signal log_added(text: String)
signal characters_loaded(characters: Array)
signal game_entered(actor_id: int, x: int, y: int, dir_light: int, hp: int, max_hp: int)
signal movement_confirmed(x: int, y: int, direction: int)
signal player_joined(actor_id: int, character_name: String, x: int, y: int, direction: int, hp: int, max_hp: int, is_dead: bool)
signal player_moved(actor_id: int, character_name: String, x: int, y: int, direction: int)
signal player_left(actor_id: int, character_name: String)
signal attack_observed(actor_id: int, from_name: String, target_actor_id: int, target_name: String, damage: int, hp: int, max_hp: int, is_dead: bool)
signal player_died(actor_id: int, character_name: String, hp: int, max_hp: int)
signal player_revived(actor_id: int, character_name: String, x: int, y: int, direction: int, hp: int, max_hp: int)
signal failed(message: String)

const TILE_WIDTH = 48
const TILE_HEIGHT = 24

@export var timeout_seconds: float = 8.0
@export var auto_create_character := true
@export var default_job := 0
@export var default_sex := 0
@export var default_hair := 0

var _peer := StreamPeerTCP.new()
var _receive_buffer := PackedByteArray()
var _busy := false
var _phase := "idle"
var _phase_elapsed := 0.0
var _pending_messages: Array[Dictionary] = []
var _connected_host := ""
var _connected_port := 0
var _account := ""
var _password := ""
var _character_name := ""
var _action := ""
var _characters := []
var _actor_id := 0
var _map_x := 0
var _map_y := 0
var _direction := 4

func is_busy() -> bool:
	return _busy and _phase != "play"

func is_in_game() -> bool:
	return _phase == "play" and _peer.get_status() == StreamPeerTCP.STATUS_CONNECTED

func query_characters(host: String, port: int, account: String, password: String, _server_name: String = "") -> void:
	_start_request(host, _server_port(port), account, password, "query", "")

func enter_game(host: String, port: int, account: String, password: String, character_name: String, _server_name: String = "") -> void:
	_start_request(host, _server_port(port), account, password, "enter", character_name.strip_edges())

func create_character(host: String, port: int, account: String, password: String, character_name: String, _server_name: String = "", job: int = 0, sex: int = 0, hair: int = 0) -> void:
	default_job = job
	default_sex = sex
	default_hair = hair
	_start_request(host, _server_port(port), account, password, "create", character_name.strip_edges())

func delete_character(_host: String, _port: int, _account: String, _password: String, _character_name: String, _server_name: String = "") -> void:
	_fail("当前简化协议暂未实现删除角色")

func say(message: String) -> void:
	if not is_in_game():
		return
	_send_json({"type": "say", "message": message})
	status_changed.emit("已发送聊天: %s" % message)

func attack(target_actor_id: int = 0) -> void:
	if not is_in_game():
		return
	_send_json({"type": "attack", "targetActorId": target_actor_id})
	status_changed.emit("已发起攻击")

func revive() -> void:
	if not is_in_game():
		return
	_send_json({"type": "revive"})
	status_changed.emit("请求复活")

func turn_to_world(world_position: Vector2, direction: int) -> void:
	if not is_in_game():
		return
	var map_position := _world_to_map(world_position)
	_send_move(int(map_position.x), int(map_position.y), direction)

func walk_to_world(world_position: Vector2, direction: int) -> void:
	if not is_in_game():
		return
	var map_position := _world_to_map(world_position)
	_send_move(int(map_position.x), int(map_position.y), direction)

func run_to_world(world_position: Vector2, direction: int) -> void:
	walk_to_world(world_position, direction)

func turn_to_map(x: int, y: int, direction: int) -> void:
	_send_move(x, y, direction)

func walk_to_map(x: int, y: int, direction: int) -> void:
	_send_move(x, y, direction)

func run_to_map(x: int, y: int, direction: int) -> void:
	_send_move(x, y, direction)

func _process(delta: float) -> void:
	if _phase == "idle" or _phase == "error" or _phase == "done":
		return
	_peer.poll()
	var status := _peer.get_status()
	if status == StreamPeerTCP.STATUS_ERROR:
		_fail("网络连接出错")
		return
	if status == StreamPeerTCP.STATUS_NONE and _phase != "done":
		_fail("网络连接已断开")
		return
	if _phase != "play":
		_phase_elapsed += delta
		if _phase_elapsed > timeout_seconds:
			_fail("请求超时: %s" % _phase)
			return
	if status == StreamPeerTCP.STATUS_CONNECTED:
		if _phase == "connecting":
			_on_connected()
		_read_available_lines()

func _start_request(host: String, port: int, account: String, password: String, action: String, character_name: String) -> void:
	_close()
	_connected_host = host.strip_edges()
	_connected_port = port
	_account = account.strip_edges()
	_password = password
	_character_name = character_name
	_action = action
	_characters.clear()
	_pending_messages.clear()
	_busy = true
	_phase_elapsed = 0.0
	_peer = StreamPeerTCP.new()
	status_changed.emit("连接 GodotMirServer %s:%d" % [_connected_host, _connected_port])
	var error := _peer.connect_to_host(_connected_host, _connected_port)
	if error != OK:
		_fail("连接服务失败: %s" % error_string(error))
		return
	_phase = "connecting"

func _on_connected() -> void:
	match _action:
		"query":
			status_changed.emit("查询角色")
			_send_json({"type": "query", "account": _account, "password": _password})
			_phase = "wait_characters"
		"create":
			if _character_name.is_empty():
				_fail("请输入要创建的角色名")
				return
			status_changed.emit("创建角色：%s" % _character_name)
			_send_json({"type": "create", "account": _account, "password": _password, "character": _character_name, "job": default_job, "sex": default_sex, "hair": default_hair})
			_phase = "wait_create"
		"enter":
			if _character_name.is_empty():
				_fail("请输入角色名")
				return
			status_changed.emit("进入游戏：%s" % _character_name)
			_send_json({"type": "enter", "account": _account, "password": _password, "character": _character_name, "job": default_job, "sex": default_sex, "hair": default_hair})
			_phase = "wait_enter"
		_:
			_fail("未知动作: %s" % _action)
	_phase_elapsed = 0.0

func _handle_message(message: Dictionary) -> void:
	var type := str(message.get("type", ""))
	match type:
		"hello":
			_log(str(message.get("message", "服务已连接")))
		"characters":
			_characters = _normalize_characters(message.get("characters", []))
			characters_loaded.emit(_characters)
			status_changed.emit("角色列表已更新，共 %d 个" % _characters.size())
			if _phase == "wait_characters" or _phase == "wait_create":
				_busy = false
				_phase = "done"
				_close()
		"created":
			status_changed.emit("角色创建成功：%s" % str(message.get("character", _character_name)))
		"entered":
			_actor_id = int(message.get("actorId", 0))
			_map_x = int(message.get("x", 330))
			_map_y = int(message.get("y", 330))
			_direction = int(message.get("direction", 4))
			_busy = true
			_phase = "play"
			status_changed.emit("成功进入游戏: X=%d Y=%d" % [_map_x, _map_y])
			game_entered.emit(_actor_id, _map_x, _map_y, _direction, int(message.get("hp", 100)), int(message.get("maxHp", 100)))
			_emit_online_players(message.get("players", []))
		"playerJoined":
			_emit_player_joined(message)
		"moved":
			var moved_actor_id := int(message.get("actorId", 0))
			var moved_character := str(message.get("character", ""))
			var moved_x := int(message.get("x", _map_x))
			var moved_y := int(message.get("y", _map_y))
			var moved_direction := int(message.get("direction", _direction))
			if moved_actor_id == _actor_id:
				_map_x = moved_x
				_map_y = moved_y
				_direction = moved_direction
				status_changed.emit("移动确认: X=%d Y=%d Dir=%d" % [_map_x, _map_y, _direction])
				movement_confirmed.emit(_map_x, _map_y, _direction)
			else:
				player_moved.emit(moved_actor_id, moved_character, moved_x, moved_y, moved_direction)
		"playerLeft":
			player_left.emit(int(message.get("actorId", 0)), str(message.get("character", "")))
		"attacked":
			attack_observed.emit(
				int(message.get("actorId", 0)),
				str(message.get("from", "")),
				int(message.get("targetActorId", 0)),
				str(message.get("target", "")),
				int(message.get("damage", 0)),
				int(message.get("hp", 0)),
				int(message.get("maxHp", 0)),
				bool(message.get("isDead", false))
			)
		"died":
			player_died.emit(int(message.get("actorId", 0)), str(message.get("character", "")), int(message.get("hp", 0)), int(message.get("maxHp", 0)))
		"revived":
			player_revived.emit(
				int(message.get("actorId", 0)),
				str(message.get("character", "")),
				int(message.get("x", 330)),
				int(message.get("y", 330)),
				int(message.get("direction", 4)),
				int(message.get("hp", 100)),
				int(message.get("maxHp", 100))
			)
		"chat":
			_log("%s: %s" % [str(message.get("from", "")), str(message.get("message", ""))])
		"error":
			_fail(str(message.get("message", "未知错误")))
		_:
			_log("忽略消息: %s" % JSON.stringify(message))

func _send_move(x: int, y: int, direction: int) -> void:
	if not is_in_game():
		return
	_map_x = max(0, x)
	_map_y = max(0, y)
	_direction = clampi(direction, 0, 7)
	_send_json({"type": "move", "x": _map_x, "y": _map_y, "direction": _direction})

func _send_json(message: Dictionary) -> void:
	if _peer.get_status() != StreamPeerTCP.STATUS_CONNECTED:
		_fail("网络连接尚未建立")
		return
	var text := JSON.stringify(message) + "\n"
	var bytes := text.to_utf8_buffer()
	var error := _peer.put_data(bytes)
	if error != OK:
		_fail("发送网络数据失败: %s" % error_string(error))

func _read_available_lines() -> void:
	var available := _peer.get_available_bytes()
	if available <= 0:
		return
	var result := _peer.get_data(available)
	if result[0] != OK:
		_fail("读取网络数据失败: %s" % error_string(result[0]))
		return
	_receive_buffer.append_array(result[1])
	while true:
		var newline := _find_byte(_receive_buffer, 10)
		if newline < 0:
			return
		var line_bytes := _receive_buffer.slice(0, newline)
		_receive_buffer = _receive_buffer.slice(newline + 1)
		var line := line_bytes.get_string_from_utf8().strip_edges()
		if line.is_empty():
			continue
		var parsed = JSON.parse_string(line)
		if typeof(parsed) != TYPE_DICTIONARY:
			_fail("服务端返回 JSON 异常: %s" % line)
			return
		_log("收到消息: %s" % line)
		_handle_message(parsed)
		if _phase == "error" or _phase == "done":
			return

func _normalize_characters(value) -> Array:
	var result := []
	if typeof(value) != TYPE_ARRAY:
		return result
	for item in value:
		if typeof(item) != TYPE_DICTIONARY:
			continue
		result.append({
			"name": str(item.get("name", "")),
			"job": int(item.get("job", 0)),
			"hair": int(item.get("hair", 0)),
			"level": int(item.get("level", 1)),
			"sex": int(item.get("sex", 0)),
			"selected": bool(item.get("selected", false)),
		})
	return result

func _emit_online_players(value) -> void:
	if typeof(value) != TYPE_ARRAY:
		return
	for item in value:
		if typeof(item) == TYPE_DICTIONARY:
			_emit_player_joined(item)

func _emit_player_joined(message: Dictionary) -> void:
	var actor_id := int(message.get("actorId", 0))
	if actor_id == 0 or actor_id == _actor_id:
		return
	player_joined.emit(
		actor_id,
		str(message.get("character", "")),
		int(message.get("x", 330)),
		int(message.get("y", 330)),
		int(message.get("direction", 4)),
		int(message.get("hp", 100)),
		int(message.get("maxHp", 100)),
		bool(message.get("isDead", false))
	)

func _world_to_map(world_position: Vector2) -> Vector2:
	var map_x := (world_position.x / TILE_WIDTH + world_position.y / TILE_HEIGHT) * 0.5
	var map_y := (world_position.y / TILE_HEIGHT - world_position.x / TILE_WIDTH) * 0.5
	return Vector2(roundi(map_x), roundi(map_y))

func _find_byte(bytes: PackedByteArray, value: int) -> int:
	for index in range(bytes.size()):
		if bytes[index] == value:
			return index
	return -1

func _server_port(port: int) -> int:
	return port

func _set_done() -> void:
	_busy = false
	_phase = "done"
	_close()

func _fail(message: String) -> void:
	_busy = false
	_phase = "error"
	status_changed.emit(message)
	failed.emit(message)
	_close()

func _log(text: String) -> void:
	log_added.emit(text)

func _close() -> void:
	if _peer.get_status() != StreamPeerTCP.STATUS_NONE:
		_peer.disconnect_from_host()
	_receive_buffer.clear()
