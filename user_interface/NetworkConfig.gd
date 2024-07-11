extends Node


# Declare member variables here. Examples:
# var a = 2
# var b = "text"
onready var host = "localhost:8080"
func get_websocket_url(client_id):
	if client_id == null:
		print("client id is null for server network")
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
	return "ws://" + host
