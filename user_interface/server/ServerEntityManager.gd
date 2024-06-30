extends EntityManagement

class_name ServerEntityManager

export var serverSpawnWorld:Resource
export var servercharacter:Resource
onready var message_controller : MessageController = MessageController.new()
onready var entity_scanner: EntityScannerTimer = EntityScannerTimer.new()
onready var terrain_scanner: TerrainScannerTimer = TerrainScannerTimer.new()

onready var server_control = get_parent() #initial node where base map is added

var spawn

func _ready():
	#socket.client_id = client_id
	entity_scanner.wait_time = 2
	entity_scanner.client_id = client_id
	entity_scanner.is_active = true
	self.add_child(entity_scanner)
	entity_scanner.start()
	
	
	terrain_scanner.wait_time = 1
	terrain_scanner.client_id = client_id
	terrain_scanner.is_active = true
	terrain_scanner.nonrelative = true
	self.add_child(terrain_scanner)
	terrain_scanner.start()
	
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

func route_to_entity(id:String,msg):
	var s = server_entities[id]
	if s!= null:
		s.message_controller.add_to_queue(msg)
		
func parseJsonCmd(cmd,delta):
	#print("raw comand:",cmd)
	var parsed = JSON.parse(cmd)
	#print("errors:",parsed.error_string)
	if parsed.result != null:
		var json:Dictionary = parsed.result
		
		match json:
			{'MSG':{'route':var route,'message':var msg}}:
				route_to_entity(route,msg)
			{'NoInput':{'id': var id}}:
				route_to_entity(id,json)
			{'Input':{"id":var id , "vec":[var x ,var y ,var z]}}:
				route_to_entity(id,json)
			{"GlobSet":{"globs":var globs}}:
				for glob in globs:
					match glob:
						{"PlayerGlob":{ "id":var id, "location" : [var x, var y, var z], "stats":{"energy": var energy,"health":var health, "id" : var discID}}}:
							if !server_entities.has(id):
								print("ServerEntityManager: creating entity , ", id , "in client id," ,client_id , spawn)
								spawn_character_entity_server(id,Vector3(x,y,z))
						{"ProwlerModel":{"id": var id, "location": [var x, var y, var z], "stats":{"energy":var energy, "health" : var health, "id": var discID}}}:
							if !server_entities.has(id):
								spawn_character_entity_server(id,Vector3(x,y,z))
						_:
							print("ServerEntityManager could not parse glob type ", glob)
			{"NextDestination":{"id": var id, "destination": var dest}}:
				route_to_entity(id,json)
			{"NEW_ENTITY": {"id":var id,"location":var location, "type": var type}}:
				pass
			{'NoLocation':{'id':var id}}:
				route_to_entity(id,json)
			{'PhysStat':{'id':var id, 'max_speed':var max_speed}}:
				#print("server entity manmager received physstat", max_speed)
				DataCache.add_data(id,'max_speed',max_speed)
			{'TerrainSet':var terrain}:
				#print("SERVER_ENTITY_terrain")
				match terrain:
					{'terrain':var t_list}:
						#print("SERVER_ENTITY_MANMAGER terrain ", t_list)
						for t in t_list:
							match t:
								{'TerrainUnitM':{'entities':var entity_map,'location':var location, 'uuid':var uuid}}:
									var keys = entity_map.keys()
									var loc = Vector3(location[0],location[1],location[2])
									for k in keys:
										var resource_id = int(k)
										var asset = AssetMapper.matchAsset(resource_id)
										for i in range(0,entity_map[k]):
											#terrain_queue.push_front({'resource_id':resource_id,'uuid':uuid,'loc':loc})
											#spawn_terrain(str(uuid),loc,spawn,asset,true)
											pass
								{'TerrainRegionM':{'terrain':var innerterain}}:
									for it in innerterain:
										match it:
											[var location,var entity_map, var uuid]:
												var keys = entity_map.keys()
												var loc = Vector3(location[0],location[1],location[2])
												for k in keys:
													var resource_id = int(k)
													var asset = AssetMapper.matchAsset(resource_id)
													for i in range(0,entity_map[k]):
														#terrain_queue.push_front({'resource_id':resource_id,'uuid':uuid,'loc':loc})
														spawn_terrain(str(uuid),loc,spawn,asset,true)
														#print("found terrain")
														pass
								_:
									print("no handler found for: ",t)
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
	
func _process(delta):
	spawn_terrain_from_queue(spawn,true)
	pass
