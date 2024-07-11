extends Node


# Declare member variables here. Examples:
# var a = 2
# var b = "text"

onready var rust_client = RustSocket.new()
# Called when the node enters the scene tree for the first time.
func _ready():
	self.add_child(rust_client)
	rust_client._client.connect("data_received",self,"handle_data")
	rust_client.connect_to_server()
	
func handle_data():
	var msg = rust_client.get_packet()
	print_debug("Message received:", msg)


# Called every frame. 'delta' is the elapsed time since the previous frame.
#func _process(delta):
#	pass
