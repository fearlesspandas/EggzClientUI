extends Control


class_name ServerControl

onready var connection_indicator:ConnectionIndicator = ConnectionIndicator.new()
onready var entity_management:ServerEntityManager = ServerEntityManager.new()
onready var auth_request:AuthenticationRequest = AuthenticationRequest.new()
onready var console:Console = Console.new()
var profile:PlayerProfile # i could remove secret from profile potentially
var connection_ind_size = 30

func _ready():
	auth_request.connect("session_created",self,"load_scene")
	self.add_child(auth_request)
	auth_request._initiate_auth_request(profile.id)
	
func load_scene(id,secret):
	profile.secret = secret
	print("entering control")
	
	entity_management.client_id = profile.id
	self.add_child(entity_management)
	ServerNetwork.init(profile.id,profile.secret,entity_management,"_on_data",false)
	console.client_id = profile.id
	
	self.add_child(console)
	ServerNetwork.bind(profile.id,console,"_on_data")
	connection_indicator.set_size(Vector2(connection_ind_size,connection_ind_size))
	connection_indicator.set_global_position(Vector2(connection_ind_size,connection_ind_size))
	connection_indicator.client_id = entity_management.client_id
	self.add_child(connection_indicator)
	
	entity_management.spawn_server_world(self,Vector3(0,-10,0))


func handle_new_entity(entity,parent,server_entity):
	print("new entity in server control")
	pass




	
