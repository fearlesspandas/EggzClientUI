extends Node
export var spawnWorld :Resource
export var serverWorld: Resource
export var maincharacter:Resource
export var servercharacter:Resource
onready var clientControl:Control = find_node("ClientControl")
onready var serverControl:Control = find_node("ServerControl")
onready var authrequest = find_node("AuthenticationRequest")
onready var authButton:Button = find_node("authButton")
onready var spawnWorldButton = find_node("SpawnWorld")
onready var newProfileId:TextEdit = find_node("NewProfileID")
onready var file_manager:GameFileManager = GameFileManager.new()
onready var profiles:Profiles = Profiles.new()
onready var spawn
onready var serverSpawn

class_name MainGame
#starting server means loading server entities which will automatically
#start updating with message traffic
#similarly starting a client is just instantiating spawn map currenly
func _ready():
	print("readying main")
	self.add_child(file_manager)
	file_manager.connect("profile_created",profiles,"create_profile_ui")
	profiles.tab_align = TabContainer.ALIGN_CENTER
	var dims = OS.get_window_safe_area()
	profiles.set_global_position(Vector2(0,0))
	profiles.set_size(dims.size)
	#profiles.rect_size = clientControl.rect_size
	self.add_child(profiles)
	spawnWorldButton.connect("button_up",self,"on_Button_button_up")
	#authButton.c
	pass

func spawn_client_world():
	spawn = EntityManager.spawn_entity("0",Vector3(0,0,0),clientControl,spawnWorld,false)
	
func spawn_server_world():
	serverSpawn = EntityManager.spawn_entity("0",Vector3(0,0,0),serverControl,serverWorld,true)

func create_character_entity_client(id:String):
	var location = Vector3(0,10,0)
	EntityManager.create_entity(id,location,spawn,maincharacter,false)

func create_character_entity_server(id:String):
	var location = Vector3(0,10,0)	
	EntityManager.create_entity(id,location,serverSpawn,servercharacter,true)
	
func _on_Button_button_up():
	if ServerNetwork.socket.connected:
		var profile:PlayerProfile = profiles.get_currently_selected_profile()
		spawn_client_world()
		create_character_entity_client(profile.id)
	else:
		print("Server is not connected")




func _on_authbutton_button_up():
	var profile:PlayerProfileUI = profiles.get_current_tab_control()
	var id = profile.profile.id
	authrequest._initiate_auth_request("1")
