extends EntityManagement

signal player_created(player)
class_name ServerEntityManager

export var serverSpawnWorld:Resource
export var servercharacter:Resource
onready var message_controller : MessageController = MessageController.new()
onready var entity_scanner: EntityScannerTimer = EntityScannerTimer.new()
onready var terrain_scanner: Timer = Timer.new()
onready var empty_terrain_queue_spawner:Timer = Timer.new()
onready var server_control = get_parent() #initial node where base map is added

var spawn

var empty_terrain_queue = []

func _ready():
	entity_scanner.wait_time = 2
	entity_scanner.client_id = client_id
	entity_scanner.is_active = true
	self.add_child(entity_scanner)
	entity_scanner.start()
	terrain_scanner.wait_time = 0.5
	terrain_scanner.connect("timeout",self,"scan_initial_terrain")
	self.add_child(terrain_scanner)
	terrain_scanner.start()
	self.add_child(message_controller)

	empty_terrain_queue_spawner.wait_time = 1
	empty_terrain_queue_spawner.connect("timeout",self,"spawn_empty_terrain_from_queue")
	self.add_child(empty_terrain_queue_spawner)
	empty_terrain_queue_spawner.start()

	pass # Replace with function body.

func spawn_empty_terrain_from_queue():
	if !empty_terrain_queue.empty():
		match empty_terrain_queue.pop_front():
			{'EmptyChunk':{'uuid':var uuid, 'location': [var x ,var y ,var z], 'radius': var radius}}:
				if true:
					var chunk = terrain[uuid]
					chunk.client_id = client_id
					chunk.uuid = uuid
					chunk.spawn = spawn
					chunk.center = Vector3(x,y,z)
					chunk.radius = radius
					chunk.is_empty = true
					chunk.is_server = true
					spawn.add_child(chunk)
					terrain[uuid] = chunk
			_:
				pass
func scan_initial_terrain():
	#socket.get_top_level_terrain()
	socket.get_top_level_terrain_in_distance(1024,Vector3(0,0,0))
	terrain_scanner.one_shot = true
	
func inspect_terrain(player:ServerEntity):
	for t in terrain.values():
		if t is Chunk and t.is_within_distance(player.body.global_transform.origin,2*t.radius):
			t.load_terrain()
			
			
func _handle_message(msg,delta_accum):
	route(msg,delta_accum)
	
func spawn_server_world(parent:Node,location:Vector3):
	print_debug("spawned server world")
	var resource = AssetMapper.matchAsset(AssetMapper.server_spawn)
	spawn = spawn_terrain("0",location,parent,resource,true)


func spawn_character_entity_server(id:String, location:Vector3) -> PlayerServerEntity:
	#print_debug("spawning server character")
	if spawn != null:
		var res:PlayerServerEntity = AssetMapper.matchAsset(AssetMapper.server_player_model).instance()
		assert(res != null)
		server_entities[id] = res
		if res.has_method("init_with_id"):
			res.init_with_id(id,client_id)
		spawn.add_child(res)
		res.global_transform.origin = location
		emit_signal("player_created",res)
		return res
	else:
		print_debug("no spawn set for server entity manager")
		return null
		
func spawn_npc_character_entity_server(id:String,location:Vector3) -> NPCServerEntity:
	var res:NPCServerEntity = AssetMapper.matchAsset(AssetMapper.npc_server_model).instance()
	server_entities[id] = res
	res.init_with_id(id,client_id)
	spawn.add_child(res)
	res.global_transform.origin = location
	emit_signal("entity_created",res,spawn,false)
	return res


func _on_data():
	var cmd = socket.get_packet(true)
	message_controller.add_to_queue(cmd)
	
func _on_physics_data():
	var cmd = physics_socket.get_packet()
	message_controller.add_to_queue(cmd)
	
func route_to_entity(id:String,msg):
	if server_entities.has(id):
		var s = server_entities[id]
		if s!= null:
			s.message_controller.add_to_queue(msg)
		
func handle_json(json) -> bool:
	match json:
			{'MSG':{'route':var route,'message':var msg}}:
				route_to_entity(route,msg)
				return false
			{'NoInput':{'id': var id}}:
				route_to_entity(id,json)
				return false
			{'Dir':{"id":var id , "vec":[var x ,var y ,var z]}}:
				route_to_entity(id,json)
				return false
			{'Input':{"id":var id , "vec":[var x ,var y ,var z]}}:
				route_to_entity(id,json)
				return false
			{"GlobSet":{"globs":var globs}}:
				var res = false
				for glob in globs:
					match glob:
						{"PlayerGlob":{ "id":var id, "location" : [var x, var y, var z], "stats":{"energy": var energy,"health":var health, "id" : var discID}}}:
							if !server_entities.has(id):
								#print_debug("ServerEntityManager: creating entity , ", id , "in client id," ,client_id , spawn)
								socket.get_top_level_terrain_in_distance(1024,Vector3(x,y,z))
								var spawned_character = spawn_character_entity_server(id,Vector3(x,y,z))
								res = true
						{"ProwlerModel":{"id": var id, "location": [var x, var y, var z], "stats":{"energy":var energy, "health" : var health, "id": var discID}}}:
							if !server_entities.has(id):
								var spawned_character = spawn_npc_character_entity_server(id,Vector3(x,y,z))
								#spawned_character.is_npc = true
						_:
							print_debug("ServerEntityManager could not parse glob type ", glob)
				return res
			{"NextDestination":{"id": var id, "destination": var dest}}:
				route_to_entity(id,json)
				return false
			{"NEW_ENTITY": {"id":var id,"location":var location, "type": var type}}:
				return false
			{'NoLocation':{'id':var id}}:
				route_to_entity(id,json)
				return false
			{'PhysStat':{'id':var id, 'max_speed':var max_speed,'speed':var speed}}:
				DataCache.add_data(id,'max_speed',max_speed)
				DataCache.add_data(id,'speed',speed)
				return false
			{'TerrainUnitm':{'entities':var entity_map,'location':var location, 'uuid':var uuid}}:
				if terrain.has(uuid):
					pass
				else:
					var keys = entity_map.keys()
					var loc = Vector3(location[0],location[1],location[2])
					for k in keys:
						var resource_id = int(k)
						assert(resource_id != 9)
						var asset = AssetMapper.matchAsset(resource_id)
						for i in range(0,entity_map[k]):
							spawn_terrain(str(uuid),loc,spawn,asset,true)
							socket.get_top_level_terrain_in_distance(1024 * 5,loc)
					terrain[uuid] = true
				return true
			{'TerrainRegionm':{'terrain':var innerterain}}:
				for it in innerterain:
					match it:
						[var location,var entity_map, var uuid]:
							var keys = entity_map.keys()
							var loc = Vector3(location[0],location[1],location[2])
							for k in keys:
								var resource_id = int(k)
								var asset = AssetMapper.matchAsset(resource_id)
								for i in range(0,entity_map[k]):
									spawn_terrain(str(uuid),loc,spawn,asset,true)
				return true
			{'TerrainChunkm': {'uuid':var uuid,'location':[var x, var y, var z], 'radius':var radius}}:
				if !terrain.has(uuid):
					var chunk = Chunk.new()
					chunk.client_id = client_id
					chunk.uuid = uuid
					chunk.spawn = spawn
					chunk.center = Vector3(x,y,z)
					chunk.radius = radius
					chunk.is_empty = false
					chunk.is_server = true
					terrain[uuid] = chunk
					spawn.add_child(chunk)
				return true
			{'EmptyChunk':{'uuid':var uuid, 'location': [var x ,var y ,var z], 'radius': var radius}}:
				if !terrain.has(uuid):
					empty_terrain_queue.push_front(json)
					var chunk = Chunk.new()
					terrain[uuid] = chunk
				return false
			_:
				print_debug("No handler found for command " , json)
				return false
func parseJsonCmd(cmd,delta):
	#print("raw comand:",cmd)
	var parsed = JSON.parse(cmd)
	#print("errors:",parsed.error_string)
	if parsed.result != null:
		var json:Dictionary = parsed.result
		if handle_json(json):
			pass
			#socket.get_next_command()
		
	else:
		#pass
		print_debug("Could not parse msg:",cmd)

func route(cmd,delta):
	#print("cmd serverentitymanager:",cmd)
	if cmd != null:
		parseJsonCmd(cmd,delta)
	
func _process(delta):
	pass
