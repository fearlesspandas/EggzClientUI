extends Spatial

class_name Destination
onready var body:MeshInstance = MeshInstance.new()
onready var mesh:CylinderMesh = CylinderMesh.new()
onready var material:SpatialMaterial = SpatialMaterial.new()
var location:Vector3

func _ready():
	material.albedo_color = Color.red
	mesh.height = 100
	mesh.top_radius = 5
	mesh.bottom_radius = 5
	self.global_transform.origin = location
	body.mesh = mesh
	body.material_override = material
	self.add_child(body)
