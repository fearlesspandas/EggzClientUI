extends Node


signal server_connected

class_name ClientWebSocket

# Our WebSocketClient instance
onready var _client = WebSocketClient.new()

onready var connected = false

var client_id
var secret
func _init():
	pass

func connect_to_server():
	var err = _client.connect_to_url(get_websocket_url(),['json'])
	if err != OK:
		connected = false
		print("Unable to connect")
		set_process(false)
func _ready():
	print("websocket client id: ", client_id)
	print("websocket secret: ",secret)
	if connected:
		pass
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
	return NetworkConfig.get_websocket_url(client_id)
func get_verification_url():
	return NetworkConfig.get_verification_url(client_id)
func _closed(was_clean = false):
	# was_clean will tell you if the disconnection was correctly notified
	# by the remote peer before closing the socket.
	print("Closed, clean: ", was_clean)
	connected = false
	set_process(false)

func _connected(proto = ""):
	# This is called on connection, "proto" will be the selected WebSocket
	# sub-protocol (which is optional)
	print("Connected with protocol: ", proto)
	connected = true
	# You MUST always use get_peer(1).put_packet to send data to server,
	# and not put_packet directly when not using the MultiplayerAPI.
	_client.get_peer(1).set_write_mode(WebSocketPeer.WRITE_MODE_TEXT)
	_client.get_peer(1).put_packet(JSON.print({'BasicSession':{'id':client_id,"secret":secret}}).to_utf8())
	print("sent session")
	emit_signal("server_connected")
	
var data = []
var delta_x = 0
func _on_data():
	# Print the received packet, you MUST always use get_peer(1).get_packet
	# to receive data from server, and not get_packet directly when not
	# using the MultiplayerAPI.
	var cmd = _client.get_peer(1).get_packet().get_string_from_utf8()
	EntityRoutingManagerClient.route(cmd,delta_x)
	data.append(cmd)
	pass
func _process(delta):
	# Call this in _process or _physics_process. Data transfer, and signals
	# emission will only happen when calling this function.
	#create_repair_egg(str(counter),"1")
	#start_egg(counter,"1")
	delta_x = delta 
	_client.poll()

func get_packet():
	return _client.get_peer(1).get_packet().get_string_from_utf8()
func create_glob(id:String,location:Vector3):
	#print("calling create glob")
	_client.get_peer(1).put_packet(JSON.print({'CREATE_GLOB':{'globId':id,'location':[location.x,location.y,location.z]}}).to_utf8())

func create_repair_egg(eggId:String,globId:String):
	_client.get_peer(1).put_packet(JSON.print({'CREATE_REPAIR_EGG':{'eggId':eggId,'globId':globId}}).to_utf8())

func get_blob(id:String):
	_client.get_peer(1).put_packet(JSON.print({'GET_BLOB':{'id':id}}).to_utf8())

func relate_eggs(id1:String,id2:String,globid:String,bidirectional:bool):
	_client.get_peer(1).put_packet(JSON.print({'RELATE_EGGS':{'egg1':id1,'egg2':id2,'globId':globid,'bidirectional':bidirectional}}).to_utf8())
	
func unrelate_eggs(id1:String,id2:String,globid:String,bidirectional:bool):
	_client.get_peer(1).put_packet(JSON.print({'UNRELATE_EGGS':{'egg1':id1,'egg2':id2,'globId':globid,'bidirectional':bidirectional}}).to_utf8())

func tick_eggs():	
	_client.get_peer(1).put_packet(JSON.print({'TICK_WORLD':{}}).to_utf8())

func getAllGlobs():
	_client.get_peer(1).put_packet(JSON.print({'GET_ALL_GLOBS':{}}).to_utf8())

func getAllEggs():
	_client.get_peer(1).put_packet(JSON.print({'GET_ALL_EGGZ':{}}).to_utf8())	

func getGlobLocation(id:String):
	_client.get_peer(1).put_packet(JSON.print({'GET_GLOB_LOCATION':{'id':str(id)}}).to_utf8())	

func setGlobLocation(id:String,location:Vector3):
	_client.get_peer(1).put_packet(JSON.print({'SET_GLOB_LOCATION':{'id':str(id),'location':[location.x,location.y,location.z]}}).to_utf8())	

func setGlobRotation(id,rotation):
	_client.get_peer(1).put_packet(JSON.print({'SET_GLOB_ROTATION':{'id':str(id),'rotation':rotation}}).to_utf8())	
				
func start_egg(eggId,globId):
	_client.get_peer(1).put_packet(JSON.print({'START_EGG':{'eggId':str(eggId),'globId':str(globId)}}).to_utf8())	

func getAllEntityIds():
	_client.get_peer(1).put_packet(JSON.print({'GET_ALL_ENTITY_IDS':{}}).to_utf8())	
	
func add_destination(globId:String,location:Vector3):
	_client.get_peer(1).put_packet(JSON.print({'ADD_DESTINATION':{'id':globId,'location':[location.x,location.y,location.z]}}).to_utf8())	
	
func get_next_destination(globId:String):
	_client.get_peer(1).put_packet(JSON.print({'GET_NEXT_DESTINATION':{'id':globId}}).to_utf8())	
	
func get_all_destinations(globId:String):
	_client.get_peer(1).put_packet(JSON.print({'GET_ALL_DESTINATIONS':{'id':globId}}).to_utf8())	
	
func location_subscribe(id:String):
	_client.get_peer(1).put_packet(JSON.print({'SUBSCRIBE':{"query":{'GET_GLOB_LOCATION':{'id':id}}}}).to_utf8())	
