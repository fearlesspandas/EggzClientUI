extends EntityManagement

class_name ClientEntityManager

onready var entity_scanner :EntityScannerTimer = EntityScannerTimer.new()
onready var message_controller : MessageController = MessageController.new()

var viewport:Viewport
var spawn
# Called when the node enters the scene tree for the first time.
func _ready():
	entity_scanner.wait_time = 2
	entity_scanner.isClient = true
	entity_scanner.client_id = client_id
	self.add_child(entity_scanner)
	
	self.add_child(message_controller)
	pass # Replace with function body.

func _handle_message(msg,delta_accum):
	route(msg,delta_accum)
	
func spawn_client_world(parent:Node,location:Vector3):
	print("spawned client world")
	var resource = AssetMapper.matchAsset(AssetMapper.client_spawn)
	spawn = spawn_terrain("0",location,parent,resource,false)

func create_character_entity_client(id:String, location:Vector3 = Vector3(0,10,0),parent = spawn):
	print("spawnging client character")
	if spawn != null:
		var resource = AssetMapper.matchAsset(AssetMapper.player_model)
		#create_entity(id,location,parent,resource,false)
		var res = spawn_player_client(id,location,parent)
		return res
	else:
		print("no spawn set for client entity manager")
		
func _on_data():
	var cmd = ServerNetwork.get(client_id).get_packet()
	message_controller.add_to_queue(cmd)



func parseJsonCmd(cmd,delta):
	#print("raw comand:",cmd)
	var parsed = JSON.parse(cmd)
	#print("errors:",parsed.error_string)
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
							if !client_entities.has(id) and client_id != id and !ServerNetwork.client_sockets.has(id):
								print("ClientEntityManager: creating entity , ", id ," in client id:",client_id, spawn)
								#ServerNetwork.bind(client_id,id,true)
								spawn_entity(id,Vector3(x,y,z),viewport,AssetMapper.matchAsset(AssetMapper.npc_model),false)
							#if id == client_id and !client_entities.has(id):
								#create_character_entity_client(id,Vector3(x,y,z),viewport)
						_:
							print("ClientEntityManager could not parse glob type ", glob)
			_:
				print("no handler found in ClientEntityManager for msg:", cmd)
	else:
		#pass
		print("Could not parse msg:",cmd)

func route(cmd,delta):
	#print("client entity manager received cmd:", cmd)
	if cmd != null:
		parseJsonCmd(cmd,delta)
	
