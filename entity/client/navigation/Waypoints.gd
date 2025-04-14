extends Node
class_name Waypoints

var ref = load("res://native_lib/Waypoints.gdns").new()
func add_parent(node):
	node.add_child(ref)
	TerrainSignalsClient.connect("add_to_nav_mesh",ref,"add_region_point")
	TerrainSignalsClient.connect("nav_mesh_bake",ref,"bake")
	TerrainSignalsClient.connect("make_nav_mesh_visible",ref,"set_chunk_visible")
	TerrainSignalsClient.connect("movement_locked",ref,"set_movement_locked")
	TerrainSignalsClient.connect("register_chunk_location",ref,"add_chunk_location")
	ClientSettings.connect("camera_render_distance",ref,"set_render_distance")
	
	#TerrainSignalsClient.connect("remove_from_nav_mesh",ref,"hide_chunk_mesh")

