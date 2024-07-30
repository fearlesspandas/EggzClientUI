extends EntityManagement

class_name ClientEntityManager

onready var entity_scanner :EntityScannerTimer = EntityScannerTimer.new()
onready var message_controller : MessageController = MessageController.new()
onready var destinations:DestinationManager = DestinationManager.new()
onready var destination_scanner : DestinationScannerTimer = DestinationScannerTimer.new()
onready var terrain_scanner : TerrainScannerTimer = TerrainScannerTimer.new()

var terrain_count = 0
var viewport:Viewport #base node where initial map is added
var spawn
func _ready():
	entity_scanner.wait_time = 2
	entity_scanner.client_id = client_id
	destination_scanner.wait_time = 1
	destination_scanner.client_id = client_id
	terrain_scanner.wait_time = 1
	terrain_scanner.client_id = client_id
	terrain_scanner.nonrelative = false
	destinations.entity_spawn = viewport
	self.add_child(entity_scanner)
	self.add_child(destination_scanner)
	self.add_child(terrain_scanner)
	self.add_child(message_controller)
	entity_scanner.start()
	destination_scanner.start()
	terrain_scanner.start()
	
func set_active(active:bool):
	entity_scanner.set_active(active)
	destination_scanner.set_active(active)
	terrain_scanner.set_active(active)

func _handle_message(msg,delta_accum):
	route(msg,delta_accum)
	
func route(cmd,delta):
	#print("client entity manager received cmd:", cmd)
	if cmd != null:
		parseJsonCmd(cmd,delta)
	
func spawn_client_world(parent:Node,location:Vector3):
	print("spawned client world")
	var resource = AssetMapper.matchAsset(AssetMapper.client_spawn)
	spawn = spawn_terrain("0",location,parent,resource,false)

func create_character_entity_client(id:String, location:Vector3 = Vector3(0,10,0),parent = spawn):
	print("spawnging client character")
	if parent != null:
		var resource = AssetMapper.matchAsset(AssetMapper.player_model)
		var res = spawn_player_client(id,location,parent)
		return res
	else:
		print("no spawn set for client entity manager")
		
func _on_data():
	var cmd = ServerNetwork.get(client_id).get_packet()
	message_controller.add_to_queue(cmd)
	
func _on_physics_data():
	var cmd = ServerNetwork.get_physics(client_id).get_packet()
	message_controller.add_to_queue(cmd)
	#print_debug("physics received: " , cmd)
	
func route_to_entity(id:String,msg):
	var s = client_entities[id]
	if s!= null:
		s.message_controller.add_to_queue(msg)
		
		
func parseJsonCmd(cmd,delta):
	var parsed = JSON.parse(cmd)
	if parsed.result != null:
		var json:Dictionary = parsed.result
		match json:
			{"SendLocation":{'id':var id, 'loc': var loc}}:
				route_to_entity(id,loc)
				pass
			{'MSG':{'route':var route,'message':var msg}}:
				route_to_entity(route,msg)
			{"NEW_ENTITY": {"id":var id,"location":var location, "type": var type}}:
				pass
			{"Location":{"id":var id, "location": [var x , var y , var z]}}:
				route_to_entity(id,json)
			{"GlobSet":{"globs":var globs}}:
				for glob in globs:
					match glob:
						{"PlayerGlob":{ "id":var id, "location" : [var x, var y, var z], "stats":{"energy": var energy,"health":var health, "id" : var discID}}}:
							#the server network check is only needed due to a bug where different players are techncially added to the same scene
							#in spite of being in different viewports
							if !client_entities.has(id) and client_id != id and (!ServerNetwork.sockets.has(id) or !ServerNetwork.physics_sockets.has(id)):
								print("ClientEntityManager: creating entity , ", id ," in client id:",client_id, spawn)
								#ServerNetwork.bind(client_id,id,true)
								spawn_entity(id,Vector3(x,y,z),viewport,AssetMapper.matchAsset(AssetMapper.npc_model),false)
						{"ProwlerModel":{"id": var id, "location": [var x, var y, var z], "stats":{"energy":var energy, "health" : var health, "id": var discID}}}:
							if !client_entities.has(id) and client_id != id and !ServerNetwork.sockets.has(id):
								spawn_entity(id,Vector3(x,y,z),viewport,AssetMapper.matchAsset(AssetMapper.npc_model),false)
						_:
							print("ClientEntityManager could not parse glob type ", glob)
							
			{"AllDestinations":{"id":var id , "destinations":var dests}}:
				destinations._handle_message(dests)
				pass
			{'LV':{'id':var id, 'lv':[var x , var y , var z]}}:
				DataCache.add_data(id,'lv',Vector3(x,y,z))
			{'PhysStat':{'id':var id, 'max_speed':var max_speed}}:
				#print("client entity manmager received physstat", max_speed)
				DataCache.add_data(id,'max_speed',max_speed)
			{'TerrainSet':var terrain}:
				match terrain:
					{'terrain':var t_list}:
						#print("CLIENT_ENTITY_MANAGER incoming terrain: ",t_list)
						for t in t_list:
							match t:
								{'TerrainUnitM':{'entities':var entity_map,'location':var location,'uuid':var uuid}}:
									var keys = entity_map.keys()
									var loc = Vector3(location[0],location[1],location[2])
									for k in keys:
										var resource_id = int(k)
										var mesh = AssetMapper.matchMesh(resource_id)
										var asset = AssetMapper.matchAsset(resource_id)
										for i in range(0,entity_map[k]):
											#terrain_queue.push_front({'resource_id':resource_id,'uuid':uuid,'loc':loc})
											terrain_count += 1
											spawn_terrain(str(uuid),loc,spawn,mesh,false)
											spawn_terrain(str(uuid),loc,spawn,asset,false)
											pass
										#print("CLIENT_ENTITY_MANAGER spawning terrain:",resource_id,loc)
								{'TerrainRegionM':{'terrain':var innerterain}}:
									for it in innerterain:
										match it:
											[var location,var entity_map, var uuid]:
												var keys = entity_map.keys()
												var loc = Vector3(location[0],location[1],location[2])
												for k in keys:
													var resource_id = int(k)
													var asset = AssetMapper.matchAsset(resource_id)
													var mesh = AssetMapper.matchMesh(resource_id)
													for i in range(0,entity_map[k]):
														#terrain_queue.push_front({'resource_id':resource_id,'uuid':uuid,'loc':loc})
														terrain_count += 1
														spawn_terrain(str(uuid),loc,spawn,mesh,false)
														spawn_terrain(str(uuid),loc,spawn,asset,false)
														pass
								{'TerrainChunkM': {'uuid':var uuid,'location':[var x, var y, var z], 'radius':var radius}}:
									if !terrain.has(uuid):
										var chunk = Chunk.new()
										chunk.client_id = client_id
										chunk.uuid = uuid
										chunk.spawn = spawn
										chunk.center = Vector3(x,y,z)
										chunk.radius = radius
										chunk.entity_manager = self
										spawn.add_child(chunk)
										terrain[uuid] = chunk
									else:
										print_debug("already has uuid , " , uuid)
									#TerrainScheduler.add_terain(chunk)
									
								_:
									print("no handler found for: ",t)
			_:						
				print("no handler found in ClientEntityManager for msg:", cmd)
	else:
		print("Could not parse msg:",cmd)


func _process(delta):
	spawn_terrain_from_queue(spawn,false)
