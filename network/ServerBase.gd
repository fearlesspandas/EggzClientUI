extends Node
class_name ServerBase
onready var entity_management:ServerEntityManager = ServerEntityManager.new()
onready var auth_request:AuthenticationRequest = AuthenticationRequest.new()

var profile_id:String
func _ready():
	Engine.physics_jitter_fix = 0
	profile_id = OS.get_environment("EGGZ_PROFILE")
	assert(profile_id != null and profile_id.length() > 0, "EGGZ_PROFILE environment variable not set")
	auth_request.connect("session_created",self,"load_scene")
	self.add_child(auth_request)
	if not ProfileManager.profile_exists(profile_id):
		print_debug("No profile found for " + profile_id + ", creating new profile")
		if ProfileManager.add_profile(profile_id) == 0:
			print_debug("successfully created profile for ", profile_id)
		else:
			print_debug("Error while creating profile for ", profile_id)
	else:
		print_debug("Profile found for ", profile_id)
	auth_request._initiate_auth_request(profile_id)

func load_scene(id,secret:String):
	SharedRuntimeEnv.initialize_sockets()
	print_debug("Starting Eggz Server with profile:",profile_id)
	var profile = ProfileManager.get_profile(profile_id)
	print_debug("profile secret: ", profile.secret)
	entity_management.client_id = profile.id
	ServerNetwork.init(profile.id,profile.secret,entity_management,"_on_data",false)
	ServerNetwork.init_physics(profile.id,profile.secret,entity_management,"_on_physics_data",false)
	self.add_child(entity_management)
	entity_management.spawn_server_world(self,Vector3(0,-10,0))
	GlobalSignalsServer.client_id_verified(profile.id)
	print_debug("Successfully started Eggz Server")
