extends Node

signal status_changed(text: String)
signal log_added(text: String)
signal characters_loaded(characters: Array)
signal game_gate_ready(host: String, port: int, certification: int)
signal game_entered(actor_id: int, x: int, y: int, dir_light: int, hp: int, max_hp: int)
signal failed(message: String)

const OpenMir2Protocol = preload("res://scripts/openmir2_protocol.gd")
const GB2312Codec = preload("res://scripts/gb2312_codec.gd")

@export var timeout_seconds: float = 10.0
@export var auto_create_character := false
@export var default_job := 0
@export var default_sex := 0
@export var default_hair := 0

var _peer := StreamPeerTCP.new()
var _receive_buffer := PackedByteArray()
var _phase := "idle"
var _phase_elapsed := 0.0
var _busy := false
var _send_num := 1
var _host := ""
var _port := 0
var _account := ""
var _password := ""
var _server_name := ""
var _character_name := ""
var _action := ""
var _sel_gate_host := ""
var _sel_gate_port := 0
var _game_gate_host := ""
var _game_gate_port := 0
var _certification := 0
var _characters := []

func is_busy() -> bool:
	return _busy

func query_characters(host: String, port: int, account: String, password: String, server_name: String = "") -> void:
	_start_login(host, port, account, password, server_name, "query", "")

func create_character(host: String, port: int, account: String, password: String, character_name: String, server_name: String = "", job: int = 0, sex: int = 0, hair: int = 0) -> void:
	default_job = job
	default_sex = sex
	default_hair = hair
	_start_login(host, port, account, password, server_name, "create", character_name.strip_edges())

func enter_game(host: String, port: int, account: String, password: String, character_name: String, server_name: String = "") -> void:
	_start_login(host, port, account, password, server_name, "enter", character_name.strip_edges())

func _process(delta: float) -> void:
	if _phase == "idle" or _phase == "done" or _phase == "error":
		return
	_peer.poll()
	var status := _peer.get_status()
	if status == StreamPeerTCP.STATUS_ERROR:
		_fail("OpenMir2 网络连接出错")
		return
	if status == StreamPeerTCP.STATUS_NONE:
		_fail("OpenMir2 网络连接已断开")
		return
	_phase_elapsed += delta
	if _phase_elapsed > timeout_seconds:
		_fail("OpenMir2 请求超时: %s" % _phase)
		return
	if status == StreamPeerTCP.STATUS_CONNECTED:
		if _phase == "login_connecting" or _phase == "sel_connecting" or _phase == "game_connecting":
			_on_connected()
		_read_available_packets()

func _start_login(host: String, port: int, account: String, password: String, server_name: String, action: String, character_name: String) -> void:
	_close()
	_host = host.strip_edges()
	_port = port
	_account = account.strip_edges()
	_password = password
	_server_name = server_name.strip_edges()
	_character_name = character_name
	_action = action
	_characters.clear()
	_sel_gate_host = ""
	_sel_gate_port = 0
	_game_gate_host = ""
	_game_gate_port = 0
	_certification = 0
	_send_num = 1
	_busy = true
	_connect_to(_host, _port, "login_connecting", "连接 OpenMir2 LoginGate %s:%d" % [_host, _port])

func _connect_to(host: String, port: int, phase: String, status_text: String) -> void:
	_close()
	_peer = StreamPeerTCP.new()
	_receive_buffer.clear()
	_phase = phase
	_phase_elapsed = 0.0
	status_changed.emit(status_text)
	var error := _peer.connect_to_host(host, port)
	if error != OK:
		_fail("连接服务失败: %s" % error_string(error))

func _on_connected() -> void:
	match _phase:
		"login_connecting":
			status_changed.emit("发送 OpenMir2 登录消息")
			_send_packet(OpenMir2Protocol.make_login_packet(_account, _password, _next_send_num()))
			_phase = "login_wait_password"
		"sel_connecting":
			status_changed.emit("查询 OpenMir2 角色")
			_send_packet(OpenMir2Protocol.make_query_character_packet(_account, _certification, _next_send_num()))
			_phase = "sel_wait_characters"
		"game_connecting":
			status_changed.emit("发送 OpenMir2 进游戏认证")
			_send_packet(OpenMir2Protocol.make_run_login_packet(_account, _character_name, _certification, _next_send_num()))
			_phase = "game_wait_logon"
	_phase_elapsed = 0.0

func _handle_packet(packet: PackedByteArray) -> void:
	var parsed := OpenMir2Protocol.parse_server_packet(packet)
	if parsed.is_empty():
		_log("忽略空 OpenMir2 包")
		return
	if parsed.get("type", "") != "message":
		_log("OpenMir2 控制包: %s" % str(parsed))
		return
	var command: Dictionary = parsed["command"]
	var body: PackedByteArray = parsed["body"]
	var ident := int(command.get("ident", 0))
	_log("OpenMir2 消息 Ident=%d Recog=%d Param=%d Tag=%d Series=%d BodyLen=%d" % [ident, int(command.get("recog", 0)), int(command.get("param", 0)), int(command.get("tag", 0)), int(command.get("series", 0)), body.size()])
	match ident:
		OpenMir2Protocol.SM_PASSOK_SELECTSERVER:
			_on_password_ok_select_server(command, body)
		OpenMir2Protocol.SM_SELECTSERVER_OK:
			_on_select_server_ok(body)
		OpenMir2Protocol.SM_QUERYCHR:
			_on_query_characters(body)
		OpenMir2Protocol.SM_QUERYCHR_FAIL:
			_fail("查询角色失败: %d" % int(command.get("recog", 0)))
		521:
			_send_packet(OpenMir2Protocol.make_query_character_packet(_account, _certification, _next_send_num()))
			_phase = "sel_wait_characters"
		522:
			_fail("创建角色失败: %d" % int(command.get("recog", 0)))
		OpenMir2Protocol.SM_STARTPLAY:
			_on_start_play(body)
		526:
			_fail("进入游戏失败，服务器可能满员")
		503:
			_fail("账号或密码错误: %d" % int(command.get("recog", 0)))
		OpenMir2Protocol.SM_LOGON:
			var actor_id := int(command.get("recog", 0))
			var x := int(command.get("param", 0))
			var y := int(command.get("tag", 0))
			var dir_light := int(command.get("series", 0))
			status_changed.emit("OpenMir2 GameGate 登录成功: X=%d Y=%d" % [x, y])
			game_gate_ready.emit(_game_gate_host, _game_gate_port, _certification)
			game_entered.emit(actor_id, x, y, dir_light, 0, 0)
			_set_done()
		_:
			_log("未处理 OpenMir2 消息: %d" % ident)

func _on_password_ok_select_server(command: Dictionary, body: PackedByteArray) -> void:
	var decoded := OpenMir2Protocol.decode_string(body)
	var chosen_server := _server_name
	if chosen_server.is_empty():
		var fields := _split_non_empty(decoded, "/")
		if fields.size() >= 1:
			chosen_server = fields[0]
	if chosen_server.is_empty():
		chosen_server = "0"
	status_changed.emit("选择 OpenMir2 服务器: %s" % chosen_server)
	_send_packet(OpenMir2Protocol.make_select_server_packet(chosen_server, _next_send_num()))
	_phase = "login_wait_select_server"
	_phase_elapsed = 0.0

func _on_select_server_ok(body: PackedByteArray) -> void:
	var decoded := OpenMir2Protocol.decode_string(body)
	var fields := _split_non_empty(decoded, "/")
	if fields.size() < 3:
		_fail("选择服务器返回异常: %s" % decoded)
		return
	_sel_gate_host = fields[0]
	_sel_gate_port = int(fields[1])
	_certification = int(fields[2])
	status_changed.emit("切换到 OpenMir2 SelGate %s:%d" % [_sel_gate_host, _sel_gate_port])
	_connect_to(_sel_gate_host, _sel_gate_port, "sel_connecting", "连接 OpenMir2 SelGate %s:%d" % [_sel_gate_host, _sel_gate_port])

func _on_query_characters(body: PackedByteArray) -> void:
	var decoded := OpenMir2Protocol.decode_string(body) if not body.is_empty() else ""
	_characters = _parse_character_list(decoded)
	characters_loaded.emit(_characters)
	status_changed.emit("OpenMir2 角色列表已更新，共 %d 个" % _characters.size())
	if _action == "query":
		_set_done()
		return
	if _action == "create":
		if _character_name.is_empty():
			_fail("请输入要创建的角色名")
			return
		_send_packet(OpenMir2Protocol.make_new_character_packet(_account, _character_name, default_hair, default_job, default_sex, _next_send_num()))
		_phase = "sel_wait_create"
		_phase_elapsed = 0.0
		return
	if _action == "enter":
		var selected := _character_name
		if selected.is_empty() and not _characters.is_empty():
			selected = str(_characters[0].get("name", ""))
		if selected.is_empty():
			if auto_create_character:
				selected = _account
				_character_name = selected
				_send_packet(OpenMir2Protocol.make_new_character_packet(_account, selected, default_hair, default_job, default_sex, _next_send_num()))
				_phase = "sel_wait_create"
				_phase_elapsed = 0.0
				return
			_fail("没有可进入的角色")
			return
		_character_name = selected
		_send_packet(OpenMir2Protocol.make_select_character_packet(_account, _character_name, _next_send_num()))
		_phase = "sel_wait_start_play"
		_phase_elapsed = 0.0

func _on_start_play(body: PackedByteArray) -> void:
	var decoded := OpenMir2Protocol.decode_string(body)
	var fields := _split_non_empty(decoded, "/")
	if fields.size() < 2:
		_fail("开始游戏返回异常: %s" % decoded)
		return
	_game_gate_host = fields[0]
	_game_gate_port = int(fields[1])
	status_changed.emit("切换到 OpenMir2 GameGate %s:%d" % [_game_gate_host, _game_gate_port])
	_connect_to(_game_gate_host, _game_gate_port, "game_connecting", "连接 OpenMir2 GameGate %s:%d" % [_game_gate_host, _game_gate_port])

func _parse_character_list(text: String) -> Array:
	var fields := text.split("/", false)
	var result := []
	var index := 0
	while index + 4 < fields.size():
		var name := str(fields[index])
		var selected := false
		if name.begins_with("*"):
			selected = true
			name = name.substr(1)
		if not name.is_empty():
			result.append({
				"name": name,
				"job": int(fields[index + 1]),
				"hair": int(fields[index + 2]),
				"level": int(fields[index + 3]),
				"sex": int(fields[index + 4]),
				"selected": selected,
			})
		index += 5
	return result

func _read_available_packets() -> void:
	var available := _peer.get_available_bytes()
	if available <= 0:
		return
	var result := _peer.get_data(available)
	if result[0] != OK:
		_fail("读取 OpenMir2 网络数据失败: %s" % error_string(result[0]))
		return
	_receive_buffer.append_array(result[1])
	while true:
		var start := _find_byte(_receive_buffer, 0x23)
		if start < 0:
			_receive_buffer.clear()
			return
		if start > 0:
			_receive_buffer = _receive_buffer.slice(start)
		var end := _find_byte(_receive_buffer, 0x21)
		if end < 0:
			return
		var packet := _receive_buffer.slice(0, end + 1)
		_receive_buffer = _receive_buffer.slice(end + 1)
		_handle_packet(packet)
		if _phase == "done" or _phase == "error":
			return

func _send_packet(packet: PackedByteArray) -> void:
	if _peer.get_status() != StreamPeerTCP.STATUS_CONNECTED:
		_fail("OpenMir2 网络连接尚未建立")
		return
	var error := _peer.put_data(packet)
	if error != OK:
		_fail("发送 OpenMir2 网络数据失败: %s" % error_string(error))

func _next_send_num() -> int:
	var current := _send_num
	_send_num += 1
	if _send_num >= 10:
		_send_num = 1
	return current

func _split_non_empty(text: String, delimiter: String) -> PackedStringArray:
	return text.split(delimiter, false)

func _find_byte(bytes: PackedByteArray, value: int) -> int:
	for index in range(bytes.size()):
		if bytes[index] == value:
			return index
	return -1

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
