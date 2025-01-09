extends MeshInstance

#doesnt seem to negatively impact performance
#if true this is a win because we can still display
#stars outside our render distance for little cost
#future updates to test
#1. include custom tags in data to distinguish between states
#2. add aggregated stats display to graph data (and maybe individualized column stats)
#3. command to save and load data snapshots into graph
#4. (stretch) diff tool to compare stats on data snapshots
var uuid
func _ready():
	GlobalSignalsClient.connect("player_position",self,"update_mesh_from_position")

func update_mesh_from_position(location:Vector3):
	var distance = (self.global_transform.origin - location).length()
	if  distance > ClientSettings.CAMERA_RENDER_DISTANCE/2 and distance <= 2*ClientSettings.CAMERA_RENDER_DISTANCE:
		TerrainSignalsClient.add_to_navigation_mesh(uuid,self.global_transform.origin, Color.blue,10)
		self.visible = false
	elif distance > 2*ClientSettings.CAMERA_RENDER_DISTANCE:
		TerrainSignalsClient.remove_from_navigation_mesh(uuid)
	else:
		self.visible = true
		TerrainSignalsClient.remove_from_navigation_mesh(uuid)

func init_with_id(id,client_id:String):
	uuid = id
