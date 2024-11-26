extends Node

signal add_to_navigation_mesh(uuid,location,color)
func add_to_navigation_mesh(uuid,location,color,radius):
	emit_signal("add_to_navigation_mesh",uuid,location,color,radius)

signal remove_from_navigation_mesh(uuid)
func remove_from_navigation_mesh(uuid):
	emit_signal("remove_from_navigation_mesh",uuid)

enum MeshState{
	NAVIGATION,
	FULLY_LOADED,
}
