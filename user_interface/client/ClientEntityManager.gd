extends EntityManagement

signal spawned_player_character(player)

class_name ClientEntityManager

onready var entity_scanner :EntityScannerTimer = EntityScannerTimer.new()
onready var message_controller : MessageController = MessageController.new()
onready var destinations:DestinationManager = DestinationManager.new()
onready var destination_scanner : DestinationScannerTimer = DestinationScannerTimer.new()

var terrain_count = 0
var viewport:Viewport #base node where initial map is added
var spawn
var is_active = false
var player:Player

func _ready():
	entity_scanner.wait_time = 3
	entity_scanner.client_id = client_id
	destination_scanner.wait_time = 10
	destination_scanner.client_id = client_id
	
	destinations.entity_spawn = viewport
	self.add_child(entity_scanner)
	self.add_child(destination_scanner)
	self.add_child(message_controller)
	entity_scanner.start()
	destination_scanner.start()
	self.connect("spawned_player_character",self,"set_player")


func set_player(player:Player):
	self.player = player
	ServerNetwork.get(client_id).get_top_level_terrain_in_distance(ClientSettings.CHUNK_DISTANCE_ON_PLAYER_LOAD,player.global_transform.origin)
	ServerNetwork.get(client_id).set_destination_mode(client_id,"FORWARD")
func set_active(active:bool):
	is_active = active
	entity_scanner.set_active(active)
	destination_scanner.set_active(active)
	#terrain_scanner.set_active(active)

func _handle_message(msg,delta_accum):
	route(msg,delta_accum)
	
func route(cmd,delta):
	#print("client entity manager received cmd:", cmd)
	if cmd != null:
		parseJsonCmd(cmd,delta)
	
func spawn_client_world(parent:Node,location:Vector3):
	print_debug("spawned client world")
	var resource = AssetMapper.matchAsset(AssetMapper.client_spawn)
	spawn = spawn_terrain("0",location,parent,resource,false)

func emit_character():
	print_debug("EMITTING SIGNAL")
	emit_signal("spawned_player_character")

func create_character_entity_client(id:String, location:Vector3 = Vector3(0,10,0),parent = spawn) -> Player:
	#print_debug("spawning client character")
	if parent != null:
		var resource = AssetMapper.matchAsset(AssetMapper.player_model)
		var res = spawn_player_client(id,location,parent)
		emit_signal("spawned_player_character",res)
		return res
	else:
		print_debug("no spawn set for client entity manager")
		return null
		
func spawn_npc_character_entity_client(id:String,location:Vector3) -> ClientPlayerEntity:
	var res:ClientPlayerEntity = AssetMapper.matchAsset(AssetMapper.npc_model).instance()
	client_entities[id] = res
	res.is_npc = true
	res.init_with_id(id,client_id)
	spawn.add_child(res)
	res.global_transform.origin = location
	emit_signal("entity_created",res,spawn,false)
	return res


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
		
func handle_json(json) -> bool:
	match json:
		{'Dir':{'id':var id, 'vec':[var x, var y , var z]}}:
			route_to_entity(id,json)
			return false
		{"SendLocation":{'id':var id, 'loc': var loc}}:
			route_to_entity(id,loc)
			return false
		{'MSG':{'route':var route,'message':var msg}}:
			route_to_entity(route,msg)
			return false
		{"NEW_ENTITY": {"id":var id,"location":var location, "type": var type}}:
			return false
		{"Location":{"id":var id, "location": [var x , var y , var z]}}:
			route_to_entity(id,json)
			return false
		{"GlobSet":{"globs":var globs}}:
			var res = false
			for glob in globs:
				match glob:
					{"PlayerGlob":{ "id":var id, "location" : [var x, var y, var z], "stats":{"energy": var energy,"health":var health, "id" : var discID}}}:
						#the server network check is only needed due to a bug where different players are techncially added to the same scene
						#in spite of being in different viewports
						if !client_entities.has(id) and id == client_id:
							print_debug("creating entity , ", id ," in client id:",client_id, spawn)
							#ServerNetwork.bind(client_id,id,true)
							#print_debug("REQUESTING CHUNKS FOR PLAYER")
							var spawned_character = create_character_entity_client(id,Vector3(x,y,z),viewport)
							#ServerNetwork.get(client_id).get_top_level_terrain_in_distance(2048,Vector3(x,y,z))
							spawned_character.set_active(self.is_active)
							res = true
							
						if !client_entities.has(id) and client_id != id and (!ServerNetwork.sockets.has(id) or !ServerNetwork.physics_sockets.has(id)):
							print_debug("creating entity , ", id ," in client id:",client_id, spawn)
							#ServerNetwork.bind(client_id,id,true)
							#ServerNetwork.get(client_id).get_top_level_terrain_in_distance(2048,Vector3(x,y,z))
							var spawned_character = spawn_entity(id,Vector3(x,y,z),viewport,AssetMapper.matchAsset(AssetMapper.npc_model),false)
							res = true
					{"ProwlerModel":{"id": var id, "location": [var x, var y, var z], "stats":{"energy":var energy, "health" : var health, "id": var discID}}}:
						if !client_entities.has(id) and client_id != id and !ServerNetwork.sockets.has(id):
							spawn_npc_character_entity_client(id,Vector3(x,y,z))
					_:
						print("ClientEntityManager could not parse glob type ", glob)
			return res
		{'ModeSet':{'mode':var mode}}:
			player.set_destination_mode(mode)
			return false
		{'ClearDestinations':{}}:
			destinations.handle_message(json)
			return false
		{'NewDestination':{'id':var id, 'destination':var dests}}:
			destinations.handle_message(json)
			return false				
		{"AllDestinations":{"id":var id , "destinations":var dests}}:
			destinations.handle_message(json)
			return false
		{'LV':{'id':var id, 'lv':[var x , var y , var z]}}:
			DataCache.add_data(id,'lv',Vector3(x,y,z))
			return false
		{'PhysStat':{'id':var id, 'max_speed':var max_speed}}:
			#print("client entity manmager received physstat", max_speed)
			DataCache.add_data(id,'max_speed',max_speed)
			return false
		{'TerrainUnitm':{'entities':var entity_map,'location':var location,'uuid':var uuid}}:
			var keys = entity_map.keys()
			var loc = Vector3(location[0],location[1],location[2])
			for k in keys:
				var resource_id = int(k)
				var mesh = AssetMapper.matchMesh(resource_id)
				var asset = AssetMapper.matchAsset(resource_id)
				for i in range(0,entity_map[k]):
					#terrain_queue.push_front({'resource_id':resource_id,'uuid':uuid,'loc':loc})
					spawn_terrain(str(uuid),loc,spawn,mesh,false)
					spawn_terrain(str(uuid),loc,spawn,asset,false)
					
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
							var mesh = AssetMapper.matchMesh(resource_id)
							for i in range(0,entity_map[k]):
								#terrain_queue.push_front({'resource_id':resource_id,'uuid':uuid,'loc':loc})
								#terrain_count += 1
								spawn_terrain(str(uuid),loc,spawn,mesh,false)
								spawn_terrain(str(uuid),loc,spawn,asset,false)
			return true
		{'TerrainChunkm': {'uuid':var uuid,'location':[var x, var y, var z], 'radius':var radius}}:
			#assert(y >= 0)
			if !terrain.has(uuid):
				var chunk = Chunk.new()
				chunk.client_id = client_id
				chunk.uuid = uuid
				chunk.spawn = spawn
				chunk.center = Vector3(x,y,z)
				chunk.radius = radius
				#chunk.player = client_entities[client_id]
				spawn.add_child(chunk)
				terrain[uuid] = chunk
				if chunk.is_within_chunk(player.body.global_transform.origin) or chunk.is_within_distance(player.body.global_transform.origin,ClientSettings.LOAD_RECEIVED_CHUNK_IF_WITHIN):
					chunk.load_terrain()
			else:
				#else case mainly handles when we run client and server in same game instancea
				var chunk = terrain[uuid]
				if chunk.is_within_chunk(player.body.global_transform.origin) or chunk.is_within_distance(player.body.global_transform.origin,ClientSettings.LOAD_RECEIVED_CHUNK_IF_WITHIN):
					chunk.load_terrain()
			return true
			
		{'TerrainSet':var terrain_set}:
			print_debug("USING OLD TERRAIN")
			match terrain_set:
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
													#terrain_count += 1
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
									chunk.player = client_entities[client_id]
									spawn.add_child(chunk)
									terrain[uuid] = chunk
								
							_:
								print_debug("no handler found for: ",t)
			return true
		_:						
			print_debug("no handler found for msg:", json)
			return false
			
func parseJsonCmd(cmd,delta):
	var parsed = JSON.parse(cmd)
	if parsed.result != null:
		var json:Dictionary = parsed.result
		if handle_json(json):
			pass
			#ServerNetwork.get(client_id).get_next_command()
	else:
		print_debug("Could not parse msg:",cmd)


func _process(delta):
	spawn_terrain_from_queue(spawn,false)
