extends EntityManagement

class_name ClientEntityManager

onready var entity_scanner :EntityScannerTimer = EntityScannerTimer.new()
onready var message_controller : MessageController = MessageController.new()
onready var destinations:DestinationManager = DestinationManager.new()
onready var destination_scanner : DestinationScannerTimer = DestinationScannerTimer.new()
var viewport:Viewport
var spawn

func _ready():
	entity_scanner.wait_time = 2
	entity_scanner.client_id = client_id
	destination_scanner.wait_time = 2
	destination_scanner.client_id = client_id
	destinations.entity_spawn = viewport
	self.add_child(entity_scanner)
	self.add_child(destination_scanner)
	self.add_child(message_controller)
	entity_scanner.start()
	destination_scanner.start()
	
func set_active(active:bool):
	entity_scanner.set_active(active)
	destination_scanner.set_active(active)

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
	if spawn != null:
		var resource = AssetMapper.matchAsset(AssetMapper.player_model)
		var res = spawn_player_client(id,location,parent)
		return res
	else:
		print("no spawn set for client entity manager")
		
func _on_data():
	var cmd = ServerNetwork.get(client_id).get_packet()
	message_controller.add_to_queue(cmd)


func parseJsonCmd(cmd,delta):
	var parsed = JSON.parse(cmd)
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
			{"GlobSet":{"globs":var globs}}:
				for glob in globs:
					match glob:
						{"PlayerGlob":{ "id":var id, "location" : [var x, var y, var z], "stats":{"energy": var energy,"health":var health, "id" : var discID}}}:
							#the server network check is only needed due to a bug where different players are techncially added to the same scene
							#in spite of being in different viewports
							if !client_entities.has(id) and client_id != id and !ServerNetwork.sockets.has(id):
								print("ClientEntityManager: creating entity , ", id ," in client id:",client_id, spawn)
								#ServerNetwork.bind(client_id,id,true)
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
			_:
				print("no handler found in ClientEntityManager for msg:", cmd)
	else:
		print("Could not parse msg:",cmd)


