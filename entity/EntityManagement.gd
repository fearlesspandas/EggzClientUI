extends Node

class_name EntityManagement
signal entity_created(entity,parent,server_entity) #Node,Node,bool
signal terrain_created(entity,parent,server_entity) #Node,Node,bool
var client_id:String
var client_entities = {}
var server_entities = {}

func create_entity(id:String,location:Vector3,parent:Node,resource:Resource,create_as_server_entity:bool):
	var res = load(resource.resource_path).instance()
	if create_as_server_entity:
		server_entities[id] = res
	else:
		client_entities[id] = res
	ServerNetwork.sockets[client_id].create_glob(id,location)
	ServerNetwork.sockets[client_id].setGlobLocation(id,location)
	parent.add_child(res)
	if res.has_method("init_with_id"):
		res.init_with_id(id)
	res.global_transform.origin = location
	emit_signal("entity_created",res,parent,create_as_server_entity)
	
func _ready():
	pass # Replace with function body.
#does not request a new entity be created serverside, whereas create_entity requests
#that a new entity is also spawned on the server
func spawn_entity(id:String,location:Vector3,parent:Node,resource:Resource,create_as_server_entity:bool):
	var res = load(resource.resource_path).instance()
	if create_as_server_entity:
		server_entities[id] = res
	else:
		client_entities[id] = res
	#ServerNetwork.setGlobLocation(id,location)
	parent.add_child(res)
	res.global_transform.origin = location
	if res.has_method("init_with_id"):
		res.init_with_id(id)
	emit_signal("terrain_created",res,parent,create_as_server_entity)
	return res
#terrain does not have message controlers associated with it
func spawn_terrain(id:String,location:Vector3,parent:Node,resource:Resource,create_as_server_entity:bool):
	var res = load(resource.resource_path).instance()
	if create_as_server_entity:
		server_entities[id] = res
	else:
		client_entities[id] = res
	#ServerNetwork.setGlobLocation(id,location)
	parent.add_child(res)
	res.global_transform.origin = location
	
	emit_signal("terrain_created",res,parent,create_as_server_entity)
	return res
	
func spawn_player_client(id:String,location:Vector3,parent:Node):
	var res = load(AssetMapper.assets[AssetMapper.player_model].resource_path).instance()
	parent.add_child(res)
	if res.has_method("init_with_id"):
		res.init_with_id(id)
	#res.camera.make_current()
	client_entities[id] = res
	res.global_transform.origin = location
