extends Control


class_name ServerControl

onready var connection_indicator:ConnectionIndicator = ConnectionIndicator.new()
onready var entity_management:ServerEntityManager = ServerEntityManager.new()
onready var auth_request:AuthenticationRequest = AuthenticationRequest.new()
onready var client_terminal = load("res://native_lib/ClientTerminal.gdns").new()
onready var server_console = ServerConsoleEnv.new()

var profile_id:String
var connection_ind_size = 30

func _ready():
	Engine.physics_jitter_fix = 0
	auth_request.connect("session_created",self,"load_scene")
	self.add_child(auth_request)
	auth_request._initiate_auth_request(profile_id)
	
func load_scene(id,secret:String):
	SharedRuntimeEnv.initialize_sockets()
	print_debug("Loading scene...")
	var profile = ProfileManager.get_profile(profile_id)
	#profile.set_secret_from_encrypted(secret)
	print_debug("profile secret: ", profile.secret)

	entity_management.client_id = profile.id
	ServerNetwork.init(profile.id,profile.secret,entity_management,"_on_data",false)
	ServerNetwork.init_physics(profile.id,profile.secret,entity_management,"_on_physics_data",false)
	self.add_child(entity_management)
	
	connection_indicator.set_size(Vector2(connection_ind_size,connection_ind_size))
	connection_indicator.set_global_position(Vector2(connection_ind_size,connection_ind_size))
	connection_indicator.client_id = entity_management.client_id
	self.add_child(connection_indicator)
	
	entity_management.spawn_server_world(self,Vector3(0,-10,0))

	print_debug("------ADDING SERVER_CONSOLE-------------------")
	self.add_child(server_console)

	#client_terminal.custom_viewport = viewport
	self.connect("is_active",client_terminal,"set_active")
	self.add_child(client_terminal)
	client_terminal.visible = false
	client_terminal.set_active(true)
	ServerTerminalGlobalSignals.register_terminal(client_terminal)


	GlobalSignalsServer.client_id_verified(profile.id)



func set_active(active:bool):
	emit_signal("is_active",active)

func handle_new_entity(entity,parent,server_entity):
	print("new entity in server control")
	pass




	
