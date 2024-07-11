extends Node


# Declare member variables here. Examples:
# var a = 2
# var b = "text"

onready var rust_client = RustSocket.new()
onready var rust_client2 = RustSocket.new()
# Called when the node enters the scene tree for the first time.
func _ready():
	self.add_child(rust_client)
	self.add_child(rust_client2)
	rust_client._client.connect("data_received",self,"handle_data")
	rust_client2._client.connect("data_received",self,"handle_data")
	rust_client.connect_to_server()
	rust_client2.connect_to_server()
	
func handle_data():
	#for i in range(0 , 100):
	var msg = rust_client.get_packet()
	var msg2 = rust_client2.get_packet()
	print_debug("Message received {client 1 : ", msg ,"}", "{client 2 : " + msg2 + "}")


# Called every frame. 'delta' is the elapsed time since the previous frame.
#func _process(delta):
#	pass