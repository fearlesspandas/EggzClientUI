extends EntityManagement

signal spawned_player_character(player)

class_name ClientEntityManager

onready var message_controller : MessageController = MessageController.new()
onready var destinations:DestinationManager = DestinationManager.new()
onready var empty_terrain_queue_spawner:Timer = Timer.new()
onready var startup_timer:Timer = Timer.new()
var terrain_count = 0
var viewport:Viewport #base node where initial map is added
var spawn
var is_active = false
var player:Player

var empty_terrain_queue = []
func _ready():
	#ready destinations
	destinations.entity_spawn = viewport
	destinations.client_id = client_id
	ClientReferences.set_destination_manager(destinations)

	#ready message controller
	self.add_child(message_controller)

	GlobalSignalsClient.connect("spawn_node",self,"spawn_node")

	#ready player
	self.connect("spawned_player_character",self,"set_player")

	#ready empty terrain queue
	empty_terrain_queue_spawner.wait_time = 0.25
	empty_terrain_queue_spawner.connect("timeout",self,"spawn_empty_terrain_from_queue")
	self.add_child(empty_terrain_queue_spawner)
	empty_terrain_queue_spawner.start()

	#ready startup timer (initial requests to setup environment)
	startup_timer.wait_time = 1
	startup_timer.connect("timeout",self,"startup_requests")
	self.add_child(startup_timer)
	startup_timer.start()

func spawn_empty_terrain_from_queue():
	#print_debug("empty terrain count : ",empty_terrain_queue.size())
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
					spawn.add_child(chunk)
					terrain[uuid] = chunk
			_:
				pass
func set_player(player:Player):
	self.player = player
	DataCache.add_data(client_id,"PLAYER",player)
	socket.get_top_level_terrain_in_distance(ClientSettings.CHUNK_DISTANCE_ON_PLAYER_LOAD,player.global_transform.origin)
	socket.set_destination_mode(client_id,"FORWARD")
	socket.set_destination_active(client_id,false)
	socket.set_gravitate(client_id,false)
	socket.get_all_destinations(client_id)
	socket.get_inventory(client_id)
	socket.getAllGlobs()
	
func split_chunks(radius,max_size):
	assert(radius > max_size)
	var num_chunk = (radius - radius%max_size)/max_size + 1
	for i in range(0,num_chunk):
		#create smaller chunk
		pass
		
func set_active(active:bool):
	is_active = active
	#destination_scanner.set_active(active)
	#terrain_scanner.set_active(active)

func startup_requests():
	startup_timer.one_shot = true
	socket.getAllGlobs()

func _handle_message(msg,delta_accum):
	route(msg,delta_accum)
	
func route(cmd,delta):
	#print("client entity manager received cmd:", cmd)
	if cmd != null:
		parseJsonCmd(cmd,delta)
	
func spawn_node(node:Spatial,location:Vector3):
	spawn.add_child(node)
	node.global_transform.origin = location

func spawn_client_world(parent:Node,location:Vector3):
	print_debug("spawned client world")
	var resource = AssetMapper.matchAsset(AssetMapper.client_spawn)
	spawn = spawn_terrain("0",location,parent,resource,false)
	AbilityManager.client_spawn = spawn

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
	var res:NonPlayerControlledEntity = AssetMapper.matchAsset(AssetMapper.npc_model).instance()
	client_entities[id] = res
	res.init_with_id(id,client_id)
	res.is_npc = true
	spawn.add_child(res)
	res.global_transform.origin = location
	emit_signal("entity_created",res,spawn,false)
	return res

func spawn_prowler_entity(id:String,location:Vector3) -> ClientPlayerEntity:
	var res:ProwlerEntity = AssetMapper.matchAsset(AssetMapper.prowler_client_entity).instance()
	client_entities[id] = res
	res.init_with_id(id,client_id)
	res.is_npc = true
	spawn.add_child(res)
	res.global_transform.origin = location
	emit_signal("entity_created",res,spawn,false)
	return res

func spawn_axis_spider(id:String,location:Vector3) -> ClientPlayerEntity:
	var res:AxisSpiderClient = AssetMapper.matchAsset(AssetMapper.axis_spider_client).instance()
	client_entities[id] = res
	res.init_with_id(id,client_id)
	res.is_npc = true
	spawn.add_child(res)
	res.global_transform.origin = location
	emit_signal("entity_created",res,spawn,false)
	return res


func _on_data():
	if socket != null:
		var cmd = socket.get_packet()
		message_controller.add_to_queue(cmd)
	
func _on_physics_data():
	var cmd = physics_socket.get_packet()
	message_controller.add_to_queue(cmd)
	#print_debug("physics received: " , cmd)
	
func route_to_entity(id:String,msg):
	if client_entities.has(id):
		var s = client_entities[id]
		if s!= null:
			s.message_controller.add_to_queue(msg)
		
func handle_globset(globs):
	for glob in globs:
		handle_entity(glob)

func handle_entity(entity):
	match entity:
		{"PlayerGlob":{ "id":var id, "location" : [var x, var y, var z], "stats":{"energy": var energy,"health":var health, "id" : var discID}}}:
			if !client_entities.has(id) and id == client_id:
				print_debug("creating entity , ", id ," in client id:",client_id, spawn)
				var spawned_character = create_character_entity_client(id,Vector3(x,y,z),viewport)
				spawned_character.set_active(self.is_active)
				spawned_character.set_health(health)
			if !client_entities.has(id) and client_id != id and (!ServerNetwork.sockets.has(id) or !ServerNetwork.physics_sockets.has(id)):
				print_debug("creating entity , ", id ," in client id:",client_id, spawn)
				var spawned_character = spawn_entity(id,Vector3(x,y,z),viewport,AssetMapper.matchAsset(AssetMapper.npc_player_entity),false)
				if spawned_character is ClientPlayerEntity:
					spawned_character.set_health(health)
		{"ProwlerModel":{"id": var id, "location": [var x, var y, var z], "stats":{"energy":var energy, "health" : var health, "id": var discID}}}:
			if !client_entities.has(id) and client_id != id and !ServerNetwork.sockets.has(id):
				var npc = spawn_prowler_entity(id,Vector3(x,y,z))
				npc.set_health(health)
		{"AxisSpiderModel":{"id": var id, "location": [var x, var y, var z], "stats":{"energy":var energy, "health" : var health, "id": var discID}}}:
			if !client_entities.has(id) and client_id != id and !ServerNetwork.sockets.has(id):
				var npc = spawn_axis_spider(id,Vector3(x,y,z))
				npc.set_health(health)
		{"MonkGardenModel":{"id":var id, "location":[var x, var y, var z],"items":var items}}:
			GlobalSignalsClient.monk_garden_received(id,Vector3(x,y,z),items)
		_:
			print_debug("no handler found for entity ", entity)


func handle_json(json) -> bool:
	match json:
		{'typ':var typ,'id':var id,'vec' : var vec}:
			route_to_entity(id,json)
			return false
		{'Loc':{'id':var id, 'loc': var loc}}:
			route_to_entity(id,loc)
			#optimize to avoid branching when handled
			GlobalSignalsClient.location_received(id)
			return false
		{'Dir':{'id':var id, 'vec':[var x, var y , var z]}}:
			route_to_entity(id,json)
			return false
		{'Rot':{'id':var id, 'vec':[var x, var y , var z]}}:
			route_to_entity(id,json)
			return false
		{'MSG':{'route':var route,'message':var msg}}:
			route_to_entity(route,msg)
			return false
		{"NEW_ENTITY": {"id":var id,"location":var location, "type": var type}}:
			return false
		{"Location":{"id":var id, "location": [var x , var y , var z]}}:
			assert(false)
			route_to_entity(id,json)
			return false
		{'Entity':{'entity':var entity}}:
			handle_entity(entity)
			return false
		{"GlobSet":{"globs":var globs}}:
			var res = false
			handle_globset(globs)
			return res
		{'ModeSet':{'mode':var mode}}:
			player.set_destination_mode(mode)
			return false
		{'DestinationsActive':{'id':var _id, 'is_active':var active}}:
			player.set_destinations_active(active)
			return false
		{'GravityActive':{'id':var _id, 'is_active':var active}}:
			player.set_gravity_active(active)
			return false
		{'ActiveDestination':{'id':var _id, 'destination':var _uuid}}:
			destinations.handle_message(json)
			return false
		{'ClearDestinations':{}}:
			destinations.handle_message(json)
			return false
		{'DeleteDestination':{'id':var _id, 'uuid':var _uuid}}:
			destinations.handle_message(json)
			return false
		{'NewDestination':{'id':var _id, 'destination':var _dests}}:
			destinations.handle_message(json)
			return false				
		{"AllDestinations":{"id":var _id , "destinations":var _dests}}:
			destinations.handle_message(json)
			return false
		{'NextIndex':{'id':var _id, 'index':var _index}}:
			destinations.handle_message(json)
			return false
		{'LV':{'id':var id, 'lv':[var x , var y , var z]}}:
			DataCache.add_data(id,'lv',Vector3(x,y,z))
			return false
		{'PhysStat':{'id':var id, 'max_speed':var max_speed,'speed':var speed}}:
			#print("client entity manmager received physstat", max_speed)
			DataCache.add_data(id,'max_speed',max_speed)
			DataCache.add_data(id,'speed',speed)
			return false
		{'AbilityAdded':{'ability_id':var ability_id,'entity_id':var id,'location':[var x , var y]}}:
			player.field.add_field_ability(int(ability_id),Vector2(x,y))
			return false
		{'AbilityRemoved':{'ability_id':var ability_id,'entity_id':var id}}:
			player.field.remove_field_ability(id,int(ability_id))
			return false
		{'DoAbility':{'ability_id':var ability_id,'entity_id':var entity_id, 'location':[var x , var y],'args' : var args}}:
			if !client_entities.has(entity_id):
				assert(false, "no entity found with id " + entity_id)
			var entity = client_entities[entity_id]
			if entity == null:
				assert(false,"entity with id is null")
			#update abilities to have location/other metadata
			var loc = entity.body.global_transform.origin
			if entity is Player:
				loc += entity.field.ref.get_point_from_location(int(x),int(y))
			AbilityManager.ability_client(int(ability_id),loc,args)	
			return false
		{'Fizzle':{'ability_id':var ability_id, 'entity_id':var entity_id,'reason': var reason}}:
			var fizzle = Fizzle.new()
			fizzle.center = player.body.global_transform.origin
			spawn.add_child(fizzle)	
			return false
		{'Inventory': {'id':var id,'items':var items}}:
			GlobalSignalsClient.inventory(id,items)	
			return false
		{'Shop': {'id':var id,'items':var items}}:
			GlobalSignalsClient.inventory(id,items)	
			return false
		{'Pocket': {'id':var id,'items':var items}}:
			GlobalSignalsClient.pocket(id,items)	
			return false
		{'AbilityPocketed':{'entity_id':var id, 'ability_id':var item,'amount':var amount}}:
			GlobalSignalsClient.pocketed_item(id,item,amount)
			return false
		{'AbilityUnpocketed':{'entity_id':var id, 'ability_id':var item,'amount':var amount}}:
			GlobalSignalsClient.unpocketed_item(id,item,amount)
			return false
		{'Field': {'id':var id,'items':var items}}:
			GlobalSignalsClient.field(id,items)	
			return false
		{'ItemAdded':{'id':var id, 'item':var item}}:
			print_debug("Item added for ", id)
			GlobalSignalsClient.item_added(id,item)
			return false
		{'ItemRemoved':{'id':var id, 'item':var item}}:
			print_debug("Item removed for ", id)
			GlobalSignalsClient.item_removed(id,item)
			return false
		{'ProgressUpdate':{'id':var id,'args':var args}}:
			ProgressHandlerClient.handle_message(id,args)
			return false
		{'TerrainUnitm':{'entities':var entity_map,'location':var location,'uuid':var uuid}}:
			if terrain.has(uuid):
				return true
			var keys = entity_map.keys()
			var loc = Vector3(location[0],location[1],location[2])
			for k in keys:
				var resource_id = int(k)
				var mesh = AssetMapper.matchMesh(resource_id)
				var asset = AssetMapper.matchClientAsset(resource_id)
				for _i in range(0,entity_map[k]):
					spawn_terrain(str(uuid),loc,spawn,mesh,false)
					spawn_terrain(str(uuid),loc,spawn,asset,false)
					socket.get_top_level_terrain_in_distance(1024 * 5,loc)
			return true
		{'TerrainRegionm':{'terrain':var innerterain}}:
			for it in innerterain:
				match it:
					[var location,var entity_map, var uuid]:
						var keys = entity_map.keys()
						var loc = Vector3(location[0],location[1],location[2])
						for k in keys:
							var resource_id = int(k)
							var asset = AssetMapper.matchClientAsset(resource_id)
							var mesh = AssetMapper.matchMesh(resource_id)
							for _i in range(0,entity_map[k]):
								var collider_terrain = spawn_terrain(str(uuid),loc,spawn,asset,false)
								var mesh_terrain = spawn_terrain(str(uuid),loc,collider_terrain,mesh,false)
								#collider_terrain.add_child(mesh_terrain)
			return true
		{'TerrainChunkm': {'uuid':var uuid,'location':[var x, var y, var z], 'radius':var radius}}:
			if !terrain.has(uuid):
				var chunk = Chunk.new()
				if ClientReferences.command_menu != null:
					ClientReferences.command_menu.toggle_chunk_visibility.connect("toggle_chunks_visible",chunk,"toggle_chunk_visibility")
				chunk.client_id = client_id
				chunk.uuid = uuid
				chunk.spawn = spawn
				chunk.center = Vector3(x,y,z)
				chunk.radius = radius
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
		{'EmptyChunk':{'uuid':var uuid, 'location': [var x ,var y ,var z], 'radius': var radius}}:
			if !terrain.has(uuid):
				empty_terrain_queue.push_front(json)
				var chunk = Chunk.new()
				terrain[uuid] = chunk
			return false
		_:						
			print_debug("no handler found for msg:", json)
			return false
			
func parseJsonCmd(cmd,delta):
	var parsed = JSON.parse(cmd)
	if parsed.result != null:
		var json = parsed.result
		if json is Dictionary:
			if handle_json(json):
				pass
			#socket.get_next_command()
		else:
			pass
			#print_debug("no handler for non dictionary result " , json)
	else:
		print_debug("Could not parse msg:",cmd)


func _process(delta):
	pass
