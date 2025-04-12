extends Node

signal add_to_navigation_mesh(uuid,location,color)
func add_to_navigation_mesh(uuid,location,color,radius):
	emit_signal("add_to_navigation_mesh",uuid,location,color,radius)

signal remove_from_navigation_mesh(uuid)
func remove_from_navigation_mesh(uuid):
	emit_signal("remove_from_navigation_mesh",uuid)

signal add_to_nav_mesh(chunk_id,terrain_id,terrain_type,location)
func add_to_nav_mesh(chunk_id,terrain_id,terrain_type,location):
	emit_signal("add_to_nav_mesh",chunk_id,terrain_id,terrain_type,location)

signal remove_from_nav_mesh(uuid)
func remove_from_nav_mesh(uuid):
	emit_signal("remove_from_nav_mesh",uuid)

signal nav_mesh_bake(uuid)
func nav_mesh_bake(uuid):
	emit_signal("nav_mesh_bake",uuid)
enum MeshState{
	NAVIGATION,
	FULLY_LOADED,
}
