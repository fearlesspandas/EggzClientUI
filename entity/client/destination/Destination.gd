extends Spatial

class_name Destination
onready var body:MeshInstance = MeshInstance.new()
onready var mesh:SphereMesh = SphereMesh.new()
onready var material:SpatialMaterial = SpatialMaterial.new()
var location:Vector3
var type:String
var radius:float

func _ready():
	material.albedo_color = Color.red
	
	mesh.radius = radius
	self.global_transform.origin = location
	body.mesh = mesh
	body.material_override = material
	self.add_child(body)
