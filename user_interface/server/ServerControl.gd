extends Control


class_name ServerControl

onready var connection_indicator:ConnectionIndicator = ConnectionIndicator.new()
onready var clientWebSocket:ClientWebSocket = ClientWebSocket.new()
onready var entity_management:ServerEntityManager = ServerEntityManager.new()
var profile:PlayerProfile = PlayerProfile.new()
var connection_ind_size = 30

func _ready():
	print("entering control")
	profile.id = "1"
	profile.secret = "SECRET"
	profile.file_location = ""
	entity_management.socket = clientWebSocket
	entity_management.client_id = profile.id
	self.add_child(entity_management)
	entity_management.start_socket(profile.secret)
	
	connection_indicator.set_size(Vector2(connection_ind_size,connection_ind_size))
	connection_indicator.set_global_position(Vector2(connection_ind_size,connection_ind_size))
	connection_indicator.socket = entity_management.socket
	self.add_child(connection_indicator)
	
	entity_management.spawn_server_world(self,Vector3(0,0,0))
	entity_management.create_character_entity_server(profile.id)

	self.add_child(clientWebSocket)

func handle_new_entity(entity,parent,server_entity):
	print("new entity in server control")
	pass
	
func start_data_transfer():
	clientWebSocket.getAllEggs()




	
