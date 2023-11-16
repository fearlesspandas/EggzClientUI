extends EntityManagement

class_name ServerEntityManager

export var serverSpawnWorld:Resource
export var servercharacter:Resource
onready var message_controller : MessageController = MessageController.new()

# Called when the node enters the scene tree for the first time.
var spawn
# Called when the node enters the scene tree for the first time.
func _ready():
	#socket.client_id = client_id
	
	self.add_child(message_controller)
	pass # Replace with function body.

func _handle_message(msg,delta_accum):
	route(msg,delta_accum)
	
func spawn_server_world(parent:Node,location:Vector3):
	print("spawned server world")
	var resource = AssetMapper.matchAsset(AssetMapper.server_spawn)
	spawn = spawn_terrain("0",location,parent,resource,true)

func create_character_entity_server(id:String):
	print("spawning server character")
	var location = Vector3(0,10,0) 
	if spawn != null:
		var resource = AssetMapper.matchAsset(AssetMapper.server_player_model)
		create_entity(id,location,spawn,resource,true)
	else:
		print("no spawn set for server entity manager")
		
func _on_data():
	var cmd = ServerNetwork.sockets[client_id].get_packet()
	message_controller.add_to_queue(cmd)

func parseJsonCmd(cmd,delta):
	#print("raw comand:",cmd)
	var parsed = JSON.parse(cmd)
	#print("errors:",parsed.error_string)
	if parsed.result != null:
		var json:Dictionary = parsed.result
		
		match json:
			{"NextDestination":{"id": var id, "location": [var x, var y , var z]}}:
				var s = server_entities[id]
				if s != null:
					var formatted = {"NextDestination":{"id":  id, "location": [ x, y , z]}}
					s.message_controller.add_to_queue(formatted)
			{"NEW_ENTITY": {"id":var id,"location":var location, "type": var type}}:
				pass
	else:
		#pass
		print("Could not parse msg:",cmd)

func route(cmd,delta):
	#print("cmd serverentitymanager:",cmd)
	if cmd != null:
		parseJsonCmd(cmd,delta)
	
