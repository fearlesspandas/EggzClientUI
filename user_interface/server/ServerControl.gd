extends Control


class_name ServerControl

onready var connection_indicator:ConnectionIndicator = ConnectionIndicator.new()
onready var clientWebSocket:ClientWebSocket = ClientWebSocket.new()
onready var entity_management:ServerEntityManager = ServerEntityManager.new()
var profile:PlayerProfile
var connection_ind_size = 30

func _ready():
	print("entering control")
	
	connection_indicator.set_size(Vector2(connection_ind_size,connection_ind_size))
	connection_indicator.set_global_position(Vector2(connection_ind_size,connection_ind_size))
	self.add_child(connection_indicator)
	
	
	
	self.add_child(entity_management)

func start_socket():
	clientWebSocket.client_id = profile.id
	clientWebSocket.secret = profile.secret
	
	#clientWebSocket.connect("server_connected",self,"start_data_transfer")
	self.add_child(clientWebSocket)
func handle_new_entity(entity,parent,server_entity):
	print("new entity in server control")
	pass
	
func start_data_transfer():
	clientWebSocket.getAllEggs()




	
