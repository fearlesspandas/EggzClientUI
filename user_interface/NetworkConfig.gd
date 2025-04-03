extends Node

signal url_set(url)
func url_set(url:String):
	emit_signal("url_set",url)

signal physics_url_set(url)
func physics_url_set(url:String):
	emit_signal("physics_url_set",url)


var host = "localhost:8080"
var physics_host = "localhost:8081"


func _ready():
	pass

func set_host(url:String):
	self.host = url
	emit_signal("url_set",url)

func set_physics_host(url:String):
	self.physics_host = url
	emit_signal("physics_url_set",url)

func get_websocket_url(client_id):
	if client_id == null:
		print_debug("client id is null for server network")
		return ""
	else:
		return "ws://" + host + "/connect/" + client_id
		
func get_verification_url(client_id):
	if client_id == null:
		print("client id is null for server network")
		return ""
	else:
		return "http://" + host + "/authenticate/" + client_id

func get_verification_url2():
	return "http://" + host + "/authenticate"

func get_rust_socket_url():
	return "ws://" + physics_host
