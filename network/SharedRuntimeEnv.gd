extends Node

onready var physics_native_shared_socket = load("res://native_lib/SharedRuntime.gdns").new()

func _ready():
	self.add_child(physics_native_shared_socket)

