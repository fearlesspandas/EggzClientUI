extends Spatial

class_name Destination

onready var body:MeshInstance = MeshInstance.new()
onready var mesh:SphereMesh = SphereMesh.new()
onready var material:SpatialMaterial = SpatialMaterial.new()

var location:Vector3
var type:String = "Empty"
var radius:float
var uuid:String
var is_empty:bool = false
var base_color:Color

func _ready():
	material.albedo_color = Color.red
	match type:
		"{WAYPOINT:{}}":
			base_color = Color.red
		"{TELEPORT:{}}":
			base_color = Color.black
	material.albedo_color = base_color
	mesh.radius = radius
	self.global_transform.origin = location
	body.mesh = mesh
	body.material_override = material
	self.add_child(body)

func highlight():
	material.albedo_color = Color.white

func unhighlight():
	material.albedo_color = base_color
