extends EntityManagement

signal player_created(player)
signal npc_created(npc)
class_name ServerEntityManager

export var serverSpawnWorld:Resource
export var servercharacter:Resource
onready var message_controller : MessageController = MessageController.new()
onready var terrain_scanner: Timer = Timer.new()
onready var empty_terrain_queue_spawner:Timer = Timer.new()
onready var server_control = get_parent() #initial node where base map is added
#onready var global_signals:GlobalSignalsServer = GlobalSignalsServer.new()

var spawn

var empty_terrain_queue = []

func _ready():
	#ready initial requests (spawn terrain and entities)
	terrain_scanner.wait_time = 0.5
	terrain_scanner.connect("timeout",self,"scan_initial_terrain")
	self.add_child(terrain_scanner)
	terrain_scanner.start()

	#ready message controller
	self.add_child(message_controller)

	#ready empty terrain queue
	empty_terrain_queue_spawner.wait_time = 1
	empty_terrain_queue_spawner.connect("timeout",self,"spawn_empty_terrain_from_queue")
	self.add_child(empty_terrain_queue_spawner)
	empty_terrain_queue_spawner.start()

	#ready global managers
	AbilityManager.client_id_server = client_id
	EntityTerrainMapper.client_id_server = client_id
	#ServerReferences.set_global_signals(global_signals)

	#self.connect("npc_created",self,"init_entity")
	pass # Replace with function body.

func init_entity(entity:NPCServerEntity):
	socket.set_destination_mode(entity.id,"FORWARD")
	socket.set_destination_active(entity.id,true)
	socket.set_gravitate(entity.id,true)

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
	socket.getAllGlobs()
	
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
	AbilityManager.server_spawn = spawn


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
		res.body.global_transform.origin = location
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
	res.body.global_transform.origin = location
	#emit_signal("entity_created",res,spawn,false)
	emit_signal("entity_created",res,spawn,true)
	emit_signal("npc_created",res)
	return res

func despawn_prowler(id:String):
	if server_entities.has(id):
		var prowler:ProwlerServerEntity = server_entities[id]
		spawn.remove_child(prowler)
		prowler.queue_free()

func despawn_spider(id:String):
	if server_entities.has(id):
		var spider : AxisSpiderServer = server_entities[id]
		spawn.remove_child(spider)
		spider.queue_free()

#spawns prowler with id at location
func spawn_prowler_character_entity_server(id:String,location:Vector3) -> ProwlerServerEntity:
	var res:ProwlerServerEntity = AssetMapper.matchAsset(AssetMapper.prowler_server_entity).instance()
	server_entities[id] = res
	res.init_with_id(id,client_id)
	spawn.add_child(res)
	res.global_transform.origin = location
	res.body.global_transform.origin = location
	GlobalSignalsServer.prowler_created(id,res)
	emit_signal("entity_created",res,spawn,true)
	emit_signal("npc_created",res)
	return res

func spawn_axis_spider(id:String,location:Vector3) -> AxisSpiderServer:
	var res:AxisSpiderServer = AssetMapper.matchAsset(AssetMapper.axis_spider_server).instance()
	server_entities[id] = res
	res.init_with_id(id,client_id)
	spawn.add_child(res)
	res.global_transform.origin = location
	res.body.global_transform.origin = location
	GlobalSignalsServer.axis_spider_created(id,res)
	emit_signal("entity_created",res,spawn,true)
	emit_signal("npc_created",res)
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
		
func handle_globset(globs):
	for glob in globs:
		handle_entity(glob)

func handle_entity(entity):
	match entity:
		{"PlayerGlob":{ "id":var id, "location" : [var x, var y, var z], "stats":{"energy": var energy,"health":var health, "id" : var discID}}}:
			if !server_entities.has(id):
				socket.get_top_level_terrain_in_distance(1024,Vector3(x,y,z))
				var spawned_character = spawn_character_entity_server(id,Vector3(x,y,z))
				#temporary - adding smack ability to all players
				socket.add_item(id,0)	
		{"ProwlerModel":{"id": var id, "location": [var x, var y, var z], "stats":{"energy":var energy, "health" : var health, "id": var discID}}}:
			if server_entities.has(id):
				despawn_prowler(id)
			var spawned_character = spawn_prowler_character_entity_server(id,Vector3(x,y,z))
		{"AxisSpiderModel":{"id": var id, "location": [var x, var y, var z], "stats":{"energy":var energy, "health" : var health, "id": var discID}}}:
			if server_entities.has(id):
				despawn_spider(id)
			var spawned_character = spawn_axis_spider(id,Vector3(x,y,z))
		_:
			print_debug("could not find handler for entity ", entity)


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
			{'Entity':{'entity':var entity}}:
				handle_entity(entity)
				return false
			{"GlobSet":{"globs":var globs}}:
				var res = false
				handle_globset(globs)
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
			{'DoAbility':{'ability_id':var ability_id,'entity_id':var entity_id, 'args': var args}}:
				if !server_entities.has(entity_id):
					assert(false, "no entity found with id " + entity_id)
				var entity = server_entities[entity_id]
				if entity == null:
					assert(false,"entity with id is null")
				AbilityManager.ability_server(int(ability_id),entity.body.global_transform.origin)	
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
						var asset = AssetMapper.matchServerAsset(resource_id)
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
								var asset = AssetMapper.matchServerAsset(resource_id)
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
