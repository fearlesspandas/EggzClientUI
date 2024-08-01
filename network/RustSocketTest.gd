extends Node


onready var rust_client = RustSocket.new()
onready var rust_client2 = RustSocket.new()
onready var auth_request = AuthenticationRequest.new()
var scala_client:ClientWebSocket

func _ready():
	rust_client.client_id = '1'
	rust_client2.client_id = '2'
	self.add_child(rust_client)
	self.add_child(rust_client2)
	rust_client._client.connect("data_received",self,"handle_data")
	rust_client2._client.connect("data_received",self,"handle_data")
	rust_client.connect_to_server()
	rust_client2.connect_to_server()
	self.add_child(auth_request)
	auth_request.connect("session_created",self,"start_client_websocket")
	auth_request._initiate_auth_request("1")
	
func handle_data():
	#for i in range(0 , 100):
	var msg = rust_client.get_packet()
	var msg2 = rust_client2.get_packet()
	print_debug("Message received {client 1 : ", msg ,"}", "{client 2 : " + msg2 + "}")

func start_client_websocket(id,secret):
	scala_client = ServerNetwork.init(id,secret,self,"handle_data")
	emit_signal("test_client_created","client_terrain_perf_test")
	
	
func _process(delta):
	pass
	rust_client.getGlobLocation(rust_client.client_id)
	rust_client2.getGlobLocation(rust_client2.client_id)


func client_terrain_perf_test():
	scala_client.get_top_level_terrain_in_distance(1000,Vector3(0,0,0))
	
	pass
