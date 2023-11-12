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
	res.global_transform.origin = location
	
func _ready():
	ServerNetwork._ready()
	pass # Replace with function body.
