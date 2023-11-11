extends Node

# The URL we will connect to
export var websocket_url = "ws://192.168.50.27:8080/subscriptions"

# Our WebSocketClient instance
var _client = WebSocketClient.new()
var counter = 0
func _ready():
	# Connect base signals to get notified of connection open, close, and errors.
	_client.connect("connection_closed", self, "_closed")
	_client.connect("connection_error", self, "_closed")
	_client.connect("connection_established", self, "_connected")
	# This signal is emitted when not using the Multiplayer API every time
	# a full packet is received.
	# Alternatively, you could check get_peer(1).get_available_packets() in a loop.
	_client.connect("data_received", self, "_on_data")

	# Initiate connection to the given URL.
	var err = _client.connect_to_url(websocket_url)
	if err != OK:
		print("Unable to connect")
		set_process(false)

func _closed(was_clean = false):
	# was_clean will tell you if the disconnection was correctly notified
	# by the remote peer before closing the socket.
	print("Closed, clean: ", was_clean)
	set_process(false)

func _connected(proto = ""):
	# This is called on connection, "proto" will be the selected WebSocket
	# sub-protocol (which is optional)
	print("Connected with protocol: ", proto)
	# You MUST always use get_peer(1).put_packet to send data to server,
	# and not put_packet directly when not using the MultiplayerAPI.
	_client.get_peer(1).set_write_mode(WebSocketPeer.WRITE_MODE_TEXT)
	create_glob('1')
	create_repair_egg('1','1')
	get_blob('1')
var data = []
func _on_data():
	# Print the received packet, you MUST always use get_peer(1).get_packet
	# to receive data from server, and not get_packet directly when not
	# using the MultiplayerAPI.
	data.append(_client.get_peer(1).get_packet().get_string_from_utf8())
	pass
func _process(delta):
	# Call this in _process or _physics_process. Data transfer, and signals
	# emission will only happen when calling this function.
	create_repair_egg(str(counter),"1")
	start_egg(counter,"1")
	if counter > 50:
		relate_eggs(str(counter -1) ,str( counter), "1")
		pass
		#tick_eggs()
	if counter % 100 == 0:
		getAllEggs()
		print(data)
		data = []
	counter += 1
	_client.poll()

func create_glob(id,location = [0.0]):
	_client.get_peer(1).put_packet(JSON.print({'CREATE_GLOB':{'globId':id,'location':location}}).to_utf8())
func create_repair_egg(eggId,globId):
	_client.get_peer(1).put_packet(JSON.print({'CREATE_REPAIR_EGG':{'eggId':eggId,'globId':globId}}).to_utf8())
func get_blob(id):
	_client.get_peer(1).put_packet(JSON.print({'GET_BLOB':{'id':id}}).to_utf8())
func relate_eggs(id1,id2,globid):
	_client.get_peer(1).put_packet(JSON.print({'RELATE_EGGS':{'egg1':id1,'egg2':id2,'globId':globid}}).to_utf8())
func tick_eggs():	
	_client.get_peer(1).put_packet(JSON.print({'TICK_WORLD':{}}).to_utf8())
func getAllGlobs():
	_client.get_peer(1).put_packet(JSON.print({'GET_ALL_GLOBS':{}}).to_utf8())

func getAllEggs():
	_client.get_peer(1).put_packet(JSON.print({'GET_ALL_STATS':{}}).to_utf8())	
	
	
func start_egg(eggId,globId):
	_client.get_peer(1).put_packet(JSON.print({'START_EGG':{'eggId':str(eggId),'globId':str(globId)}}).to_utf8())	
