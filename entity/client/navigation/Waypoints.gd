extends Node

var ref = load("res://native_lib/Waypoints.gdns").new()
func add_parent(node):
	node.add_child(ref)
	TerrainSignalsClient.connect("add_to_nav_mesh",ref,"add_terrain")
	TerrainSignalsClient.connect("remove_from_nav_mesh",ref,"hide_chunk_mesh")
