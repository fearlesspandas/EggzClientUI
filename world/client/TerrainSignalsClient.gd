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

signal make_nav_mesh_visible(chunk_id,is_visible)
func make_nav_mesh_visible(chunk_id:String,is_visible:bool):
	emit_signal("make_nav_mesh_visible",chunk_id,is_visible)

signal remove_from_nav_mesh(uuid)
func remove_from_nav_mesh(uuid):
	emit_signal("remove_from_nav_mesh",uuid)

signal nav_mesh_bake(uuid)
func nav_mesh_bake(uuid):
	emit_signal("nav_mesh_bake",uuid)

signal movement_locked(uuid,is_locked)
func movement_locked(uuid,is_locked):
	emit_signal("movement_locked",uuid,is_locked)

signal register_chunk_location(chunk_id,location)
func register_chunk_location(chunk_id,location):
	emit_signal("register_chunk_location",chunk_id,location)

enum MeshState{
	NAVIGATION,
	FULLY_LOADED,
}
