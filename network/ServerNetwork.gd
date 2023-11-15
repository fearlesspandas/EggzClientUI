extends Node


onready var socket:ClientWebSocket

func _ready():
	pass
	
func init(id,secret):
	print("readying server")
	socket =  ClientWebSocket.new()
	socket.client_id = id
	socket.secret = secret
	self.add_child(socket)

