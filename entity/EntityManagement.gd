extends Node

class_name EntityManagement
signal entity_created(entity,parent,server_entity) #Node,Node,bool
signal terrain_created(entity,parent,server_entity) #Node,Node,bool
var client_id:String
var client_entities = {}
var server_entities = {}
#var terrain_queue = []
var terrain = {}
var socket:ClientWebSocket
var physics_socket:RustSocket

func _ready():
	assert(client_id != null)
	socket = ServerNetwork.get(client_id)
	assert(socket != null)
	physics_socket = ServerNetwork.get_physics(client_id)
	assert(physics_socket != null)

func spawn_entity(id:String,location:Vector3,parent:Node,resource:Resource,create_as_server_entity:bool):
	var res = load(resource.resource_path).instance()
	if create_as_server_entity:
		server_entities[id] = res
	else:
		client_entities[id] = res
	if res.has_method("init_with_id"):
		res.init_with_id(id,client_id)
	parent.add_child(res)
	res.global_transform.origin = location
	emit_signal("entity_created",res,parent,create_as_server_entity)
	return res

#terrain does not have message controlers associated with it
func spawn_terrain(id:String,location:Vector3,parent:Node,resource:Resource,create_as_server_entity:bool):
	if resource != null:
		var res = resource.instance()
		terrain[id] = res
		parent.add_child(res)
		res.global_transform.origin = location
		if res.has_method("init_with_id"):
			res.init_with_id(id)
		emit_signal("terrain_created",res,parent,create_as_server_entity)
		return res
	else:
		return null

func spawn_player_client(id:String,location:Vector3,parent:Node):
	var res:Player = AssetMapper.matchAsset(AssetMapper.player_model).instance()
	if res.has_method("init_with_id"):
		res.init_with_id(id,client_id)
	parent.add_child(res)
	#res.camera.make_current()
	client_entities[id] = res
	res.global_transform.origin = location
	return res

