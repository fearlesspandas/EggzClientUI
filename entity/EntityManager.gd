extends Node



var client_entities = {}
var server_entities = {}

func create_entity(id:String,location:Vector3,parent:Node,resource:Resource,create_as_server_entity:bool):
	var res = load(resource.resource_path).instance()
	
	
	if create_as_server_entity:
		server_entities[id] = res
	else:
		client_entities[id] = res
	ServerNetwork.create_glob(id,location)
	#ServerNetwork.setGlobLocation(id,location)
	parent.add_child(res)
	if res.has_method("init_with_id"):
		res.init_with_id(id)
	res.global_transform.origin = location
	
func _ready():
	ServerNetwork._ready()
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
	return res
	
