extends Spatial
class_name NavigatorMesh

var vertices = {}

func _ready():
	TerrainSignalsClient.connect("add_to_navigation_mesh",self,"add_terrain_to_mesh")
	TerrainSignalsClient.connect("remove_from_navigation_mesh",self,"remove_terrain_from_mesh")

func add_terrain_to_mesh(uuid,location,color):
	if !vertices.has(uuid):
		vertices[uuid] = add_nav_point(location,color)

func remove_terrain_from_mesh(uuid):
	if vertices.has(uuid):
		var m = vertices[uuid]
		self.remove_child(m)
		m.queue_free()
		vertices.erase(uuid)

func add_nav_point(anchor:Vector3,color:Color) -> NavigationPointMesh:
	var nav_mesh = NavigationPointMesh.new()
	nav_mesh.anchor = anchor
	nav_mesh.color = color
	nav_mesh.distance = 500
	nav_mesh.radius = 4
	self.add_child(nav_mesh)
	return nav_mesh
