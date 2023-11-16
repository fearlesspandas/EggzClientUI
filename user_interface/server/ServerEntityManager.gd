extends EntityManagement

class_name ServerEntityManager

export var serverSpawnWorld:Resource
export var servercharacter:Resource
onready var message_controller : MessageController = MessageController.new()

# Called when the node enters the scene tree for the first time.
onready var client_id
var spawn
# Called when the node enters the scene tree for the first time.
func _ready():
	#socket.client_id = client_id
	
	self.add_child(message_controller)
	pass # Replace with function body.

func start_socket(secret:String):
	socket.client_id = client_id 
	socket.secret = secret
	self.add_child(self.socket)
	self.socket._client.connect("data_received", self, "_on_data")
	self.socket.connect_to_server()
	message_controller.socket = self.socket
func _handle_message(msg,delta_accum):
	route(msg,delta_accum)
	print("Server entity manager received message")
	
func spawn_server_world(parent:Node,location:Vector3):
	print("spawned client world")
	var resource = AssetMapper.matchAsset(AssetMapper.server_spawn)
	spawn = spawn_terrain("0",location,parent,resource,false)

func create_character_entity_server(id:String):
	print("spawnging client character")
	var location = Vector3(0,10,0) 
	if spawn != null:
		var resource = AssetMapper.matchAsset(AssetMapper.server_player_model)
		create_entity(id,location,spawn,resource,false)
	else:
		print("no spawn set for client entity manager")
		
func _on_data():
	var cmd = socket.get_packet()
	message_controller.add_to_queue(cmd)





func parseJsonCmd(cmd,delta):
	#print("raw comand:",cmd)
	var parsed = JSON.parse(cmd)
	#print("errors:",parsed.error_string)
	if parsed.result != null:
		var json:Dictionary = parsed.result
		
		match json:
			{"NEW_ENTITY": {"id":var id,"location":var location, "type": var type}}:
				pass
			{"Location":{"id":var id, "location": [var x , var y , var z]}}:
				var s = client_entities[id]
				if s != null:
					var formatted = {"Location":{"id":id, "location": [ x , y , z]}}
					s.message_controller.add_to_queue(formatted)
	else:
		#pass
		print("Could not parse msg:",cmd)

func route(cmd,delta):
	if cmd != null:
		parseJsonCmd(cmd,delta)
	
