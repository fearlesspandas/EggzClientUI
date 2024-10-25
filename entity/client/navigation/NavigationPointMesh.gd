extends Spatial

class_name NavigationPointMesh

var color:Color
var distance:float = 10
var anchor:Vector3

var radius:float
func _ready():
	assert(distance > 0)
	assert(radius > 0)
	assert(color != null and color != Color())
	assert(anchor != null )
	var p : MeshInstance = MeshInstance.new()
	p.mesh = PointMesh.new()
	p.mesh.material = SpatialMaterial.new()
	p.mesh.material.albedo_color = color
	p.mesh.material.flags_use_point_size = true
	p.mesh.material.params_point_size = radius
	p.mesh.material.params_billboard_keep_scale = false
	p.transform.origin = Vector3(0,0,-distance) 
	self.look_at(anchor,Vector3.UP)
	self.add_child(p)

func _process(delta):
	self.look_at(anchor,Vector3.UP)


