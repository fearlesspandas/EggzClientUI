extends Node


var sockets = {}
func _ready():
	pass

func bind(id,handler:Node,method:String):
	if sockets.has(id):
		var socket:ClientWebSocket = sockets[id]
		socket._client.connect("data_received",handler,method)
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
		
func get(id) -> ClientWebSocket:
	if sockets.has(id):
		return sockets[id]
	else:
		return null
