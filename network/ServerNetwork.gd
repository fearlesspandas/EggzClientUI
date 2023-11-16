extends Node


var sockets = {}

func _ready():
	pass
	
func init(id,secret, handler:Node,method):
	print("readying server for session ", id)
	var socket =  ClientWebSocket.new()
	socket.client_id = id
	socket.secret = secret
	self.add_child(socket)
	socket._client.connect("data_received", handler, "_on_data")
	sockets[id] = socket
	socket.connect_to_server()
	
