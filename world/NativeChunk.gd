extends Node
class_name NativeChunk

var ref:Area = load("res://native_lib/Chunk.gdns").new()
#onready var parent = get_parent()
var client_id

var registered = false
func add_parent(par):
	ref.input_ray_pickable = false
	par.add_child(ref)
	par.add_child(self)
	ref.connect("load_terrain",self,"load_terrain")
	ref.connect("fill_empty_terrain",self,"fill_empty_terrain")

	#GlobalSignalsClient.connect("player_position",self,"update_mesh_from_position")

func _process(delta):
	self.set_process(false)
	if ref.server():
		self.set_process(false)
	if registered && ref.sent_all():
		assert(false)
		return
	if registered:
		register_nav_mesh_locations()
		return
	register_chunk_to_nav_mesh()
	registered = true
	
func load_terrain(terrain_id:String,radius:float):
	#print_debug("Body Entered Native Chunk: ",str(terrain_id))
	ServerNetwork.get(client_id).get_cached_terrain(terrain_id)
	var distance = ClientSettings.CHUNK_REQUEST_RADIUS_MULTIPLIER*radius
	ServerNetwork.get(client_id).get_top_level_terrain_in_distance(distance,ref.global_transform.origin)

func fill_empty_terrain(terrain_id,entity_id):
	#print_debug("Body Entered Empty Native Chunk:",str(terrain_id)," ",str(entity_id))
	ServerNetwork.get(client_id).fill_empty_chunk(terrain_id,entity_id)

func register_chunk_to_nav_mesh():
	var chunk_id:String = ref.id()
	var location:Vector3 = ref.global_transform.origin
	TerrainSignalsClient.register_chunk_location(chunk_id,location)
	

func update_mesh_from_position(location:Vector3):
	if ref.server():
		return

	var distance = (self.ref.global_transform.origin - location).length()
	if  distance > ClientSettings.CAMERA_RENDER_DISTANCE:
		register_nav_mesh_locations()
	else:
		var chunk_id = ref.id()
		#TerrainSignalsClient.movement_locked(chunk_id,false)
		TerrainSignalsClient.make_nav_mesh_visible(chunk_id,false)
		ref.set_hidden(false)
		pass
		#ref.set_point_mesh(false);
		#ref.commit_point_mesh();
	if  distance > 2*ClientSettings.CAMERA_RENDER_DISTANCE:
		pass
		#var chunk_id = ref.id()
		#TerrainSignalsClient.movement_locked(chunk_id,true)
	
func register_nav_mesh_locations():
	var res = ref.send_locations();
	ref.update_cached_send()
	var chunk_id = res[0]
	var locations = res[1]
	var should_bake = false
	for data in locations:
		match data:
			[var terrain_id, var terrain_type,var location]:
				if terrain_id is String and terrain_type is int and location is Vector3:
					should_bake = true
					TerrainSignalsClient.add_to_nav_mesh(chunk_id,terrain_id,terrain_type,location)
					pass
				else:
					assert(false)
			_:
				assert(false)
	if should_bake:
		TerrainSignalsClient.nav_mesh_bake(chunk_id)
	TerrainSignalsClient.make_nav_mesh_visible(chunk_id,true)
	ref.set_hidden(true)

