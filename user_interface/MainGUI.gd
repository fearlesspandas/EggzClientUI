extends Node
export var spawnWorld :Resource
export var serverWorld: Resource
export var maincharacter:Resource
export var servercharacter:Resource
onready var clientControl = find_node("ClientControl")
onready var serverControl = find_node("ServerControl")
onready var authrequest = find_node("AuthenticationRequest")
onready var spawn
onready var serverSpawn

class_name MainGame
#starting server means loading server entities which will automatically
#start updating with message traffic
#similarly starting a client is just instantiating spawn map currenly
func _ready():
	print("readying main")
	pass

func spawn_client_world():
	spawn = EntityManager.spawn_entity("0",Vector3(0,0,0),clientControl,spawnWorld,false)
	
func spawn_server_world():
	serverSpawn = EntityManager.spawn_entity("0",Vector3(0,0,0),serverControl,serverWorld,true)

func create_character_entity_client():
	var location = Vector3(0,10,0)
	EntityManager.create_entity("1",location,spawn,maincharacter,false)

func create_character_entity_server():
	var location = Vector3(0,10,0)	
	EntityManager.create_entity("1",location,serverSpawn,servercharacter,true)

func _on_Button_button_up():
	if ServerNetwork.socket.connected:
		spawn_client_world()
		create_character_entity_client()
	else:
		print("Server is not connected")




func _on_authbutton_button_up():
	authrequest._initiate_auth_request("1")
