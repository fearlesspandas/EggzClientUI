extends Node


var server_sockets = {}
var server_binds = {}
var client_sockets = {}
var client_binds = {}
func _ready():
	pass
	
func init(id,secret, handler:Node,method,isClient:bool = true):
	print("readying server for session ", id)
	var socket =  ClientWebSocket.new()
	socket.client_id = id
	socket.secret = secret
	self.add_child(socket)
	socket._client.connect("data_received", handler, "_on_data")
	if isClient:
		client_sockets[id] = socket
	else:
		server_sockets[id] = socket
	socket.connect_to_server()
	
func bind(source,target,isClient:bool = true):
	print("binding:",source,target,isClient)
	if isClient:
		client_binds[target] = source
	else:
		server_binds[target] = source
		
func get(id,isClient:bool = true) -> ClientWebSocket:
	if isClient:
		return client_sockets[id]
	else:
		return server_sockets[id]
