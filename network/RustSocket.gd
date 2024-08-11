extends Node


signal server_connected

class_name RustSocket

# Our WebSocketClient instance
onready var _client = WebSocketClient.new()

onready var connected = false

var client_id
var secret
func _init():
	pass

func connect_to_server():
	var url = get_websocket_url()
	print_debug("attempting to connect with url ", url)
	var err = _client.connect_to_url(url)
	print_debug("has err ", err)
	if err != OK:
		connected = false
		print("Unable to connect")
		set_process(false)
		
func _ready():
	print_debug("initializing socket")
	# Connect base signals to get notified of connection open, close, and errors.
	_client.connect("connection_closed", self, "_closed")
	_client.connect("connection_error", self, "_closed")
	_client.connect("connection_established", self, "_connected")
	# This signal is emitted when not using the Multiplayer API every time
	# a full packet is received.
	# Alternatively, you could check get_peer(1).get_available_packets() in a loop.
	
	#print("removed old data received in ClientWebSocket")
	# Initiate connection to the given URL.
	

func get_websocket_url():
	return NetworkConfig.get_rust_socket_url()
	
func get_verification_url():
	return NetworkConfig.get_verification_url(client_id)
	
func _closed(was_clean = false):
	# was_clean will tell you if the disconnection was correctly notified
	# by the remote peer before closing the socket.
	print_debug("Closed, clean: ", was_clean)
	connected = false
	set_process(false)

func _connected(proto = ""):
	# This is called on connection, "proto" will be the selected WebSocket
	# sub-protocol (which is optional)
	print_debug("Connected with protocol: ", proto)
	connected = true
	# You MUST always use get_peer(1).put_packet to send data to server,
	# and not put_packet directly when not using the MultiplayerAPI.
	_client.get_peer(1).set_write_mode(WebSocketPeer.WRITE_MODE_TEXT)
	#_client.get_peer(1).put_packet(JSON.print({'BasicSession':{'id':client_id,"secret":secret}}).to_utf8())
	#setGlobLocation(client_id,Vector3(0,0,0))
	print_debug("sent session")
	emit_signal("server_connected")
	
var delta_x = 0

func _process(delta):
	delta_x = delta 
	_client.poll()
	
var last_packet = null
func get_packet(use_default:bool = false):
	var res = _client.get_peer(1).get_packet().get_string_from_utf8()
	if (res == null or res.length() == 0 ) and use_default:
		return last_packet
	else:
		last_packet = res
		return res
	
func send_payload(payload):
	_client.get_peer(1).put_packet(JSON.print(payload).to_utf8())
	
func create_glob(id:String,location:Vector3):
	#print("calling create glob")
	_client.get_peer(1).put_packet(JSON.print(PayloadMapper.create_glob(id,location)).to_utf8())

func create_repair_egg(eggId:String,globId:String):
	_client.get_peer(1).put_packet(JSON.print(PayloadMapper.create_repair_egg(eggId,globId)).to_utf8())

func get_blob(id:String):
	_client.get_peer(1).put_packet(JSON.print(PayloadMapper.get_blob(id)).to_utf8())

func relate_eggs(id1:String,id2:String,globid:String,bidirectional:bool):
	_client.get_peer(1).put_packet(JSON.print(PayloadMapper.relate_eggs(id1,id2,globid,bidirectional)).to_utf8())
	
func unrelate_eggs(id1:String,id2:String,globid:String,bidirectional:bool):
	_client.get_peer(1).put_packet(JSON.print(PayloadMapper.unrelate_eggs(id1,id2,globid,bidirectional)).to_utf8())

func tick_eggs():	
	_client.get_peer(1).put_packet(JSON.print(PayloadMapper.tick_eggs()).to_utf8())

func getAllGlobs():
	_client.get_peer(1).put_packet(JSON.print(PayloadMapper.getAllGlobs()).to_utf8())

func getAllEggs():
	_client.get_peer(1).put_packet(JSON.print(PayloadMapper.getAllEggs()).to_utf8())	

func getGlobLocation(id:String):
	_client.get_peer(1).put_packet(JSON.print(PayloadMapper.getGlobLocation(id)).to_utf8())	

func get_location_physics(id:String):
	send_payload(PayloadMapper.getLocationPhysics(id))

func setGlobLocation(id:String,location:Vector3):
	_client.get_peer(1).put_packet(JSON.print(PayloadMapper.setGlobLocation(id,location)).to_utf8())	

func set_location_physics(id:String,location:Vector3):
	send_payload(PayloadMapper.setLocationPhysics(id,location))

func setGlobRotation(id,rotation):
	_client.get_peer(1).put_packet(JSON.print(PayloadMapper.setGlobRotation(id,rotation)).to_utf8())	
				
func start_egg(eggId,globId):
	_client.get_peer(1).put_packet(JSON.print(PayloadMapper.start_egg(eggId,globId)).to_utf8())	

func getAllEntityIds():
	_client.get_peer(1).put_packet(JSON.print(PayloadMapper.getAllEntityIds()).to_utf8())	
	
func add_destination(globId:String,location:Vector3,type:String,radius:float):
	_client.get_peer(1).put_packet(JSON.print(PayloadMapper.add_destination(globId,location,type,radius)).to_utf8())
	
func get_next_destination(globId:String):
	_client.get_peer(1).put_packet(JSON.print(PayloadMapper.get_next_destination(globId)).to_utf8())
	
func get_all_destinations(globId:String):
	_client.get_peer(1).put_packet(JSON.print(PayloadMapper.get_all_destinations(globId)).to_utf8())
	
func location_subscribe(id:String):
	_client.get_peer(1).put_packet(JSON.print(PayloadMapper.location_subscribe(id)).to_utf8())

func input_subscribe(id:String):
	_client.get_peer(1).put_packet(JSON.print(PayloadMapper.input_subscribe(id)).to_utf8())
	
func get_input_physics(id:String):
	send_payload(PayloadMapper.get_input_physics(id))
	
func get_dir_physics(id:String):
	send_payload(PayloadMapper.get_dir_physics(id))

func send_input(id:String,inputVec:Vector3):	
	_client.get_peer(1).put_packet(JSON.print(PayloadMapper.set_input_physics(id,inputVec)).to_utf8())

func clear_destinations(id:String):
	_client.get_peer(1).put_packet(JSON.print(PayloadMapper.clear_destinations(id)).to_utf8())

func set_lv(id:String,lv:Vector3):
	_client.get_peer(1).put_packet(JSON.print(PayloadMapper.set_lv(id,lv)).to_utf8())

func lazy_lv(id:String):
	_client.get_peer(1).put_packet(JSON.print(PayloadMapper.lazy_lv(id)).to_utf8())

func adjust_stats(id:String,delta):
	_client.get_peer(1).put_packet(JSON.print(PayloadMapper.adjust_stats(id,delta)).to_utf8())

func get_physical_stats(id:String):
	_client.get_peer(1).put_packet(JSON.print(PayloadMapper.get_physical_stats(id)).to_utf8())

func subscribe_general(query):
	_client.get_peer(1).put_packet(JSON.print(PayloadMapper.subscribe_general(query)).to_utf8())

func get_all_terrain(id:String,nonrelative:bool):
	send_payload(PayloadMapper.get_all_terrain(id,nonrelative))
	
func create_terrain(id:String,location:Vector3):
	send_payload(PayloadMapper.create_terrain(id,location))

func get_all_terrain_within_player_distance(id:String,radius:float):
	send_payload(PayloadMapper.get_terrain_within_player_distance(id,radius))
