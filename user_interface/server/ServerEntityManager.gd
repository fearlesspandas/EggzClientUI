extends EntityManagement

class_name ServerEntityManager

export var serverSpawnWorld:Resource
export var servercharacter:Resource
onready var message_controller : MessageController = MessageController.new()
onready var entity_scanner: EntityScannerTimer = EntityScannerTimer.new()
# Called when the node enters the scene tree for the first time.
var spawn
# Called when the node enters the scene tree for the first time.
func _ready():
	#socket.client_id = client_id
	entity_scanner.wait_time = 2
	entity_scanner.isClient = false
	entity_scanner.client_id = client_id
	entity_scanner.is_active = true
	self.add_child(entity_scanner)
	entity_scanner.start()
	message_controller.isClient = false
	self.add_child(message_controller)
	pass # Replace with function body.

func _handle_message(msg,delta_accum):
	route(msg,delta_accum)
	
func spawn_server_world(parent:Node,location:Vector3):
	print("spawned server world")
	var resource = AssetMapper.matchAsset(AssetMapper.server_spawn)
	spawn = spawn_terrain("0",location,parent,resource,true)

func create_character_entity_server(id:String, location:Vector3):
	print("spawning server character")
	if spawn != null:
		var resource = AssetMapper.matchAsset(AssetMapper.server_player_model)
		create_entity(id,location,spawn,resource,true)
	else:
		print("no spawn set for server entity manager")
		
func spawn_character_entity_server(id:String, location:Vector3):
	print("spawning server character")
	if spawn != null:
		var resource = AssetMapper.matchAsset(AssetMapper.server_player_model)
		return spawn_entity(id,location,spawn,resource,true)
	else:
		print("no spawn set for server entity manager")
		
func _on_data():
	var cmd = ServerNetwork.get(client_id,false).get_packet()
	message_controller.add_to_queue(cmd)

func parseJsonCmd(cmd,delta):
	#print("raw comand:",cmd)
	var parsed = JSON.parse(cmd)
	#print("errors:",parsed.error_string)
	if parsed.result != null:
		var json:Dictionary = parsed.result
		
		match json:
			{"GlobSet":{"globs":var globs}}:
				for glob in globs:
					match glob:
						{"PlayerGlob":{ "id":var id, "location" : [var x, var y, var z], "stats":{"energy": var energy,"health":var health, "id" : var discID}}}:
							if !server_entities.has(id):
								print("ServerEntityManager: creating entity , ", id , "in client id," ,client_id , spawn)
								spawn_character_entity_server(id,Vector3(x,y,z))
						_:
							print("ServerEntityManager could not parse glob type ", glob)
			{"NextDestination":{"id": var id, "location": [var x, var y , var z]}}:
				var s = server_entities[id]
				if s != null:
					var formatted = {"NextDestination":{"id":  id, "location": [ x, y , z]}}
					s.message_controller.add_to_queue(formatted)
			{"NEW_ENTITY": {"id":var id,"location":var location, "type": var type}}:
				pass
			_:
				print("no matching command in ServerEntityManager for ", cmd)
	else:
		#pass
		print("Could not parse msg:",cmd)

func route(cmd,delta):
	#print("cmd serverentitymanager:",cmd)
	if cmd != null:
		parseJsonCmd(cmd,delta)
	
