extends Node
class_name NativeChunk

var ref:Area = load("res://native_lib/Chunk.gdns").new()
#onready var parent = get_parent()
var client_id

func add_parent(par):
	ref.input_ray_pickable = false
	par.add_child(ref)
	ref.connect("load_terrain",self,"load_terrain")
	ref.connect("fill_empty_terrain",self,"fill_empty_terrain")
	ref.connect("location",self,"send_location")
	GlobalSignalsClient.connect("player_position",self,"update_mesh_from_position")


func load_terrain(terrain_id:String,radius:float):
	#print_debug("Body Entered Native Chunk: ",str(terrain_id))
	ServerNetwork.get(client_id).get_cached_terrain(terrain_id)
	var distance = ClientSettings.CHUNK_REQUEST_RADIUS_MULTIPLIER*radius
	ServerNetwork.get(client_id).get_top_level_terrain_in_distance(distance,ref.global_transform.origin)

func fill_empty_terrain(terrain_id,entity_id):
	#print_debug("Body Entered Empty Native Chunk:",str(terrain_id)," ",str(entity_id))
	ServerNetwork.get(client_id).fill_empty_chunk(terrain_id,entity_id)

func update_mesh_from_position(location:Vector3):
	var distance = (self.ref.global_transform.origin - location).length()
	if  distance > ClientSettings.CAMERA_RENDER_DISTANCE/2 and distance <= 2*ClientSettings.CAMERA_RENDER_DISTANCE:
		ref.set_point_mesh(true);
		ref.commit_point_mesh();
	elif distance > 2*ClientSettings.CAMERA_RENDER_DISTANCE:
		ref.set_point_mesh(false);
		ref.commit_point_mesh();
	else:
		ref.set_point_mesh(false);
		ref.commit_point_mesh();

func send_location(region_id:String,terrain_type:int,location:Vector3):
	TerrainSignalsClient.add_nav_to_mesh(region_id,terrain_type,location)
	
func remove_terrain_mesh(region_id):
	TerrainSignalsClient.remove_from_nav_mesh(region_id)
	
