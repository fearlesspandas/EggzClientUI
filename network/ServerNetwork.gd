extends Node


onready var socket:ClientWebSocket

func _ready():
	pass
	
func init(id,secret):
	print("readying server")
	socket =  ClientWebSocket.new()
	socket.client_id = id
	socket.secret = secret
	socket.connect("server_connected",self,"start_data_transfer")
	self.add_child(socket)
	
	#get server entities on startup

func start_data_transfer():
	socket.getAllEggs()
