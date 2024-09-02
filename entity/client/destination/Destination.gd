extends Spatial

class_name Destination

onready var body:MeshInstance = MeshInstance.new()
onready var mesh:SphereMesh = SphereMesh.new()
onready var material:SpatialMaterial = SpatialMaterial.new()

var location:Vector3
var type:String
var radius:float
var uuid:String
var is_empty:bool = false
func _ready():
	material.albedo_color = Color.red
	match type:
		"{WAYPOINT:{}}":
			material.albedo_color = Color.red
		"{TELEPORT:{}}":
			material.albedo_color = Color.black
	mesh.radius = radius
	self.global_transform.origin = location
	body.mesh = mesh
	body.material_override = material
	self.add_child(body)
