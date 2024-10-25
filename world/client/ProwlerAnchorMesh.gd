extends MeshInstance
class_name ProwlerAnchorMesh

var uuid
var loaded:bool = false
func _ready():
	GlobalSignalsClient.connect("player_position",self,"update_mesh_from_position")

func update_mesh_from_position(location:Vector3):
	if (self.global_transform.origin - location).length() > ClientSettings.CAMERA_RENDER_DISTANCE :
		TerrainSignalsClient.add_to_navigation_mesh(uuid,self.global_transform.origin, Color.red)
		loaded = false
	else:
		TerrainSignalsClient.remove_from_navigation_mesh(uuid)
		loaded = true
	self.visible = loaded

func init_with_id(id):
	uuid = id
