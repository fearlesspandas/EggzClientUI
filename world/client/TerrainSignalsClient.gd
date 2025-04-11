extends Node

signal add_to_navigation_mesh(uuid,location,color)
func add_to_navigation_mesh(uuid,location,color,radius):
	emit_signal("add_to_navigation_mesh",uuid,location,color,radius)

signal remove_from_navigation_mesh(uuid)
func remove_from_navigation_mesh(uuid):
	emit_signal("remove_from_navigation_mesh",uuid)

signal add_to_nav_mesh(uuid,terrain_type,location)
func add_to_nav_mesh(uuid,location,terrain_type):
	emit_signal("add_to_nav_mesh",uuid,terrain_type,location)

signal remove_from_nav_mesh(uuid)
func remove_from_nav_mesh(uuid):
	emit_signal("remove_from_nav_mesh",uuid)

enum MeshState{
	NAVIGATION,
	FULLY_LOADED,
}
