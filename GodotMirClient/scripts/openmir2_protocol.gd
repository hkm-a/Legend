extends RefCounted

const GB2312Codec = preload("res://scripts/gb2312_codec.gd")

const DEF_BLOCK_SIZE := 16
const COMMAND_SIZE := 12
const CLIENT_VERSION_NUMBER := 1200409180

const CM_QUERYCHR := 100
const CM_NEWCHR := 101
const CM_DELCHR := 102
const CM_SELCHR := 103
const CM_SELECTSERVER := 104
const CM_IDPASSWORD := 2001
const CM_LOGINNOTICEOK := 1018
const CM_TURN := 3010
const CM_WALK := 3011
const CM_RUN := 3013
const CM_HIT := 3014
const CM_SAY := 3030

const SM_LOGON := 50
const SM_NEWMAP := 51
const SM_QUERYCHR := 520
const SM_STARTPLAY := 525
const SM_QUERYCHR_FAIL := 527
const SM_PASSOK_SELECTSERVER := 529
const SM_SELECTSERVER_OK := 530

const _BY_SEED := 0xAC
const _BY_BASE := 0x3C

static func make_message(ident: int, recog: int = 0, param: int = 0, tag: int = 0, series: int = 0) -> Dictionary:
	return {
		"recog": recog,
		"ident": ident & 0xFFFF,
		"param": param & 0xFFFF,
		"tag": tag & 0xFFFF,
		"series": series & 0xFFFF,
	}

static func encode_message(message: Dictionary) -> PackedByteArray:
	return encode_buffer(message_to_bytes(message))

static func decode_message(encoded: PackedByteArray) -> Dictionary:
	return bytes_to_message(decode_buffer(encoded))

static func encode_string(text: String) -> PackedByteArray:
	return encode_buffer(GB2312Codec.encode(text))

static func decode_string(encoded: PackedByteArray) -> String:
	return GB2312Codec.decode(decode_buffer(encoded))

static func encode_buffer(source: PackedByteArray, pos: int = 0) -> PackedByteArray:
	var result := PackedByteArray()
	var no := 2
	var remainder := 0
	var index: int = maxi(0, pos)
	while index < source.size():
		var c := (int(source[index]) ^ _BY_SEED) & 0xFF
		if no == 6:
			result.append(((c & 0x3F) + _BY_BASE) & 0xFF)
			remainder = (remainder | ((c >> 2) & 0x30)) & 0xFF
			result.append((remainder + _BY_BASE) & 0xFF)
			remainder = 0
		else:
			var temp := c >> 2
			result.append((((temp & 0x3C) | (c & 0x03)) + _BY_BASE) & 0xFF)
			remainder = ((remainder << 2) | (temp & 0x03)) & 0xFF
		no = no % 6 + 2
		index += 1
	if no != 2:
		result.append((remainder + _BY_BASE) & 0xFF)
	return result

static func decode_buffer(source: PackedByteArray) -> PackedByteArray:
	var result := PackedByteArray()
	var cycles := int(source.size() / 4)
	var bytes_left := source.size() % 4
	for cycle in range(cycles):
		var begin := cycle * 4
		var remainder := int(source[begin + 3]) - _BY_BASE
		var temp := int(source[begin]) - _BY_BASE
		var c := (((temp << 2) & 0xF0) | (remainder & 0x0C) | (temp & 0x03)) & 0xFF
		result.append((c ^ _BY_SEED) & 0xFF)
		temp = int(source[begin + 1]) - _BY_BASE
		c = (((temp << 2) & 0xF0) | ((remainder << 2) & 0x0C) | (temp & 0x03)) & 0xFF
		result.append((c ^ _BY_SEED) & 0xFF)
		temp = int(source[begin + 2]) - _BY_BASE
		c = (temp | ((remainder << 2) & 0xC0)) & 0xFF
		result.append((c ^ _BY_SEED) & 0xFF)
	if bytes_left == 2:
		var remainder := int(source[source.size() - 1]) - _BY_BASE
		var temp := int(source[source.size() - 2]) - _BY_BASE
		var c := (((temp << 2) & 0xF0) | ((remainder << 2) & 0x0C) | (temp & 0x03)) & 0xFF
		result.append((c ^ _BY_SEED) & 0xFF)
	elif bytes_left == 3:
		var remainder := int(source[source.size() - 1]) - _BY_BASE
		var temp := int(source[source.size() - 3]) - _BY_BASE
		var c := (((temp << 2) & 0xF0) | (remainder & 0x0C) | (temp & 0x03)) & 0xFF
		result.append((c ^ _BY_SEED) & 0xFF)
		temp = int(source[source.size() - 2]) - _BY_BASE
		c = (((temp << 2) & 0xF0) | ((remainder << 2) & 0x0C) | (temp & 0x03)) & 0xFF
		result.append((c ^ _BY_SEED) & 0xFF)
	return result

static func message_to_bytes(message: Dictionary) -> PackedByteArray:
	var bytes := PackedByteArray()
	_write_i32_le(bytes, int(message.get("recog", 0)))
	_write_u16_le(bytes, int(message.get("ident", 0)))
	_write_u16_le(bytes, int(message.get("param", 0)))
	_write_u16_le(bytes, int(message.get("tag", 0)))
	_write_u16_le(bytes, int(message.get("series", 0)))
	return bytes

static func bytes_to_message(bytes: PackedByteArray, offset: int = 0) -> Dictionary:
	if bytes.size() - offset < COMMAND_SIZE:
		return {}
	return make_message(
		_read_u16_le(bytes, offset + 4),
		_read_i32_le(bytes, offset),
		_read_u16_le(bytes, offset + 6),
		_read_u16_le(bytes, offset + 8),
		_read_u16_le(bytes, offset + 10)
	)

static func make_client_packet(message: Dictionary, body: PackedByteArray = PackedByteArray(), send_num: int = 1) -> PackedByteArray:
	var payload := PackedByteArray()
	payload.append_array(encode_message(message))
	payload.append_array(body)
	return wrap_client_payload(payload, send_num)

static func wrap_client_payload(payload: PackedByteArray, send_num: int = 1) -> PackedByteArray:
	var packet := PackedByteArray()
	packet.append(0x23)
	packet.append_array(GB2312Codec.encode(str(clampi(send_num, 1, 9))))
	packet.append_array(payload)
	packet.append(0x21)
	return packet

static func wrap_server_payload(payload: PackedByteArray) -> PackedByteArray:
	var packet := PackedByteArray()
	packet.append(0x23)
	packet.append_array(payload)
	packet.append(0x21)
	return packet

static func parse_server_packet(packet: PackedByteArray) -> Dictionary:
	var payload := unwrap_packet(packet)
	if payload.is_empty():
		return {}
	if payload[0] == 0x2B:
		return {"type": "control", "text": GB2312Codec.decode(payload)}
	if payload.size() < DEF_BLOCK_SIZE:
		return {"type": "short", "payload": payload}
	var command := decode_message(payload.slice(0, DEF_BLOCK_SIZE))
	var body := payload.slice(DEF_BLOCK_SIZE)
	return {
		"type": "message",
		"command": command,
		"body": body,
	}

static func unwrap_packet(packet: PackedByteArray) -> PackedByteArray:
	var start := 0
	var end := packet.size()
	if end > 0 and packet[0] == 0x23:
		start = 1
	if end > start and packet[end - 1] == 0x21:
		end -= 1
	return packet.slice(start, end)

static func make_login_packet(account: String, password: String, send_num: int = 1) -> PackedByteArray:
	return make_client_packet(make_message(CM_IDPASSWORD), encode_string(account + "/" + password), send_num)

static func make_select_server_packet(server_name: String, send_num: int = 1) -> PackedByteArray:
	return make_client_packet(make_message(CM_SELECTSERVER), encode_string(server_name), send_num)

static func make_query_character_packet(account: String, certification: int, send_num: int = 1) -> PackedByteArray:
	return make_client_packet(make_message(CM_QUERYCHR), encode_string(account + "/" + str(certification)), send_num)

static func make_new_character_packet(account: String, character_name: String, hair: int, job: int, sex: int, send_num: int = 1) -> PackedByteArray:
	var body := "%s/%s/%d/%d/%d" % [account, character_name, hair, job, sex]
	return make_client_packet(make_message(CM_NEWCHR), encode_string(body), send_num)

static func make_select_character_packet(account: String, character_name: String, send_num: int = 1) -> PackedByteArray:
	return make_client_packet(make_message(CM_SELCHR), encode_string(account + "/" + character_name), send_num)

static func make_run_login_packet(account: String, character_name: String, certification: int, send_num: int = 1, client_version: int = CLIENT_VERSION_NUMBER, build_number: int = 2022080300) -> PackedByteArray:
	var body := "**%s/%s/%d/%d/%d" % [account, character_name, certification, client_version, build_number]
	return wrap_client_payload(encode_string(body), send_num)

static func make_notice_ok_packet(tick: int, send_num: int = 1) -> PackedByteArray:
	return make_client_packet(make_message(CM_LOGINNOTICEOK, tick), PackedByteArray(), send_num)

static func make_turn_packet(x: int, y: int, direction: int, send_num: int = 1) -> PackedByteArray:
	return make_client_packet(make_message(CM_TURN, 0, x, y, direction), PackedByteArray(), send_num)

static func make_walk_packet(x: int, y: int, direction: int, send_num: int = 1) -> PackedByteArray:
	return make_client_packet(make_message(CM_WALK, 0, x, y, direction), PackedByteArray(), send_num)

static func make_run_packet(x: int, y: int, direction: int, send_num: int = 1) -> PackedByteArray:
	return make_client_packet(make_message(CM_RUN, 0, x, y, direction), PackedByteArray(), send_num)

static func make_hit_packet(x: int, y: int, direction: int, send_num: int = 1) -> PackedByteArray:
	return make_client_packet(make_message(CM_HIT, 0, x, y, direction), PackedByteArray(), send_num)

static func make_say_packet(message: String, send_num: int = 1) -> PackedByteArray:
	return make_client_packet(make_message(CM_SAY), encode_string(message), send_num)

static func _write_i32_le(bytes: PackedByteArray, value: int) -> void:
	var unsigned := value & 0xFFFFFFFF
	bytes.append(unsigned & 0xFF)
	bytes.append((unsigned >> 8) & 0xFF)
	bytes.append((unsigned >> 16) & 0xFF)
	bytes.append((unsigned >> 24) & 0xFF)

static func _write_u16_le(bytes: PackedByteArray, value: int) -> void:
	var unsigned := value & 0xFFFF
	bytes.append(unsigned & 0xFF)
	bytes.append((unsigned >> 8) & 0xFF)

static func _read_i32_le(bytes: PackedByteArray, offset: int) -> int:
	var value := int(bytes[offset]) | (int(bytes[offset + 1]) << 8) | (int(bytes[offset + 2]) << 16) | (int(bytes[offset + 3]) << 24)
	if value >= 0x80000000:
		return value - 0x100000000
	return value

static func _read_u16_le(bytes: PackedByteArray, offset: int) -> int:
	return int(bytes[offset]) | (int(bytes[offset + 1]) << 8)
