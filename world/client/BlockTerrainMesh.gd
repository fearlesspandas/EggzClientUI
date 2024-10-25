extends MeshInstance

#var uuid
#func _ready():
#	GlobalSignalsClient.connect("player_position",self,"update_mesh_from_position")
#
#func update_mesh_from_position(location:Vector3):
#	if (self.global_transform.origin - location).length() > ClientSettings.CAMERA_RENDER_DISTANCE:
#		TerrainSignalsClient.add_to_navigation_mesh(uuid,self.global_transform.origin, Color.blue)
#	else:
#		TerrainSignalsClient.remove_from_navigation_mesh(uuid)
#
#func init_with_id(id):
#	uuid = id
