extends Node
class_name Waypoints

var ref = load("res://native_lib/Waypoints.gdns").new()
func add_parent(node):
	node.add_child(ref)
	TerrainSignalsClient.connect("add_to_nav_mesh",ref,"add_region_point")
	TerrainSignalsClient.connect("nav_mesh_bake",ref,"bake")
	#TerrainSignalsClient.connect("remove_from_nav_mesh",ref,"hide_chunk_mesh")

