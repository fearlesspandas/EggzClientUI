extends Node

class_name MessageController

onready var parent = get_parent()

var isClient:bool = true

func add_to_queue(msg):
	if parent.has_method("_handle_message"):
		parent._handle_message(msg,ServerNetwork.get(parent.client_id,isClient).delta_x)


func _ready():
	pass

