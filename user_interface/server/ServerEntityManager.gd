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
	entity_scanner.client_id = client_id
	entity_scanner.is_active = true
	self.add_child(entity_scanner)
	entity_scanner.start()
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
	var cmd = ServerNetwork.get(client_id).get_packet(true)
	message_controller.add_to_queue(cmd)

func parseJsonCmd(cmd,delta):
	#print("raw comand:",cmd)
	var parsed = JSON.parse(cmd)
	#print("errors:",parsed.error_string)
	if parsed.result != null:
		var json:Dictionary = parsed.result
		
		match json:
			{'NoInput':{'id': var id}}:
				var s = server_entities[id]
				if s!= null:
					s.message_controller.add_to_queue(
							{'NoInput':{'id' : id}}
						)
			{'Input':{"id":var id , "vec":[var x ,var y ,var z]}}:
				var s = server_entities[id]
				if s!= null:
					s.message_controller.add_to_queue(
							{'Input':{"id":id , "vec":[ x , y ,z]}}
						)
				pass
			{"GlobSet":{"globs":var globs}}:
				for glob in globs:
					match glob:
						{"PlayerGlob":{ "id":var id, "location" : [var x, var y, var z], "stats":{"energy": var energy,"health":var health, "id" : var discID}}}:
							if !server_entities.has(id):
								print("ServerEntityManager: creating entity , ", id , "in client id," ,client_id , spawn)
								spawn_character_entity_server(id,Vector3(x,y,z))
						_:
							print("ServerEntityManager could not parse glob type ", glob)
			{"NextDestination":{"id": var id, "destination": var dest}}:
				var s = server_entities[id]
				if s != null:
					var formatted = {"NextDestination":{"id":  id, 'destination':dest}}
					s.message_controller.add_to_queue(formatted)
			{"NEW_ENTITY": {"id":var id,"location":var location, "type": var type}}:
				pass
			{'NoLocation':{'id':var id}}:
				var s = server_entities[id]
				if s != null:
					var formatted = {'NoLocation':{'id':id}}
					s.message_controller.add_to_queue(formatted)
			{'PhysStat':{'id':var id, 'max_speed':var max_speed}}:
				#print("server entity manmager received physstat", max_speed)
				DataCache.add_data(id,'max_speed',max_speed)
			_:
				pass
				#print("no matching command in ServerEntityManager for ", cmd)
	else:
		#pass
		print("Could not parse msg:",cmd)

func route(cmd,delta):
	#print("cmd serverentitymanager:",cmd)
	if cmd != null:
		parseJsonCmd(cmd,delta)
	
