extends Node


var sockets = {}
var physics_sockets = {}

func _ready():
	pass

func bind(id,handler:Node,method:String):
	if sockets.has(id):
		var socket:ClientWebSocket = sockets[id]
		socket._client.connect("data_received",handler,method)
#initializes web socket for client with Id		
func init(id,secret, handler:Node,method:String,isClient:bool = true) -> ClientWebSocket:
	print("readying server for session ", id)
	var socket =  ClientWebSocket.new()
	socket.client_id = id
	socket.secret = secret
	self.add_child(socket)
	socket._client.connect("data_received", handler, method)
	sockets[id] = socket
	socket.connect_to_server()
	return socket
		
func init_physics(id,secret, handler:Node,method:String,isClient:bool = true) -> RustSocket:
	print("readying server for session ", id)
	var physics_socket = RustSocket.new()
	physics_socket.client_id = id
	self.add_child(physics_socket)
	physics_socket._client.connect("data_received", handler, method)
	physics_sockets[id] = physics_socket
	physics_socket.connect_to_server()
	return physics_socket
		
func get(id) -> ClientWebSocket:
	if sockets.has(id):
		return sockets[id]
	else:
		return null
		
func get_physics(id) -> RustSocket:
	if physics_sockets.has(id):
		return physics_sockets[id]
	else:
		return null
