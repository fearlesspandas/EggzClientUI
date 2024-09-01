extends Node

class_name EntityManagement
signal entity_created(entity,parent,server_entity) #Node,Node,bool
signal terrain_created(entity,parent,server_entity) #Node,Node,bool
var client_id:String
var client_entities = {}
var server_entities = {}
var terrain_queue = []
var terrain = {}

func _ready():
	pass # Replace with function body.

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
	var res = resource.instance()
	terrain[id] = res
	parent.add_child(res)
	res.global_transform.origin = location
	if res.has_method("init_with_id"):
		res.init_with_id(id)
	emit_signal("terrain_created",res,parent,create_as_server_entity)
	return res

func spawn_terrain_from_queue(spawn,server:bool = false):
	if !terrain_queue.empty():
		var t = terrain_queue.pop_front()
		var resource_id = t.resource_id
		var asset = AssetMapper.matchAsset(resource_id)
		if !server:
			var mesh = AssetMapper.matchMesh(resource_id)
			spawn_terrain(str(t.uuid),t.loc,spawn,mesh,server)
		spawn_terrain(str(t.uuid),t.loc,spawn,asset,server)
		
func spawn_player_client(id:String,location:Vector3,parent:Node):
	var res:Player = AssetMapper.matchAsset(AssetMapper.player_model).instance()
	if res.has_method("init_with_id"):
		res.init_with_id(id,client_id)
	parent.add_child(res)
	#res.camera.make_current()
	client_entities[id] = res
	res.global_transform.origin = location
	return res

