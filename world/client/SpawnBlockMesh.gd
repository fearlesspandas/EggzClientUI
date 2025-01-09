extends Spatial

var uuid
var loaded:bool = false
func _ready():
	var re = RotatingEntities.new()
	re.center_point = Vector3(0,10,0)
	re.radius = 500
	self.add_child(re)
	GlobalSignalsClient.connect("player_position",self,"update_mesh_from_position")

func update_mesh_from_position(location:Vector3):
	if (self.global_transform.origin - location).length() > ClientSettings.CAMERA_RENDER_DISTANCE :
		loaded = false
		TerrainSignalsClient.add_to_navigation_mesh(uuid,self.global_transform.origin, Color.purple,128)
	else:
		loaded = true
		TerrainSignalsClient.remove_from_navigation_mesh(uuid)
	self.visible = loaded

func init_with_id(id,client_id:String):
	uuid = id
