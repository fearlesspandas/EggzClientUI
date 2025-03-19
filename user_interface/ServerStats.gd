extends Node

class_name ServerStats

onready var ref = load("res://native_lib/ServerStats.gdns").new()

onready var parent = get_parent()

func _ready():
	parent.add_child(ref)
	SharedRuntimeEnv.connect("connected",self,"connected")
	SharedRuntimeEnv.connect("disconnected",self,"disconnected")

func connected(socket_id:int,entities:Array):
	ref.add_connection(socket_id,entities)
	ref.set_connection_status(socket_id,true)

func disconnected(socket_id:int,_entities:Array):
	ref.set_connection_status(socket_id,false)

