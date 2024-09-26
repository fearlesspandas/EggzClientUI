extends StaticBody

class_name TowerChunk

onready var mesh_instance:MeshInstance = MeshInstance.new()

onready var collision_shape:CollisionShape = CollisionShape.new()

var radius = 100
var height = 10

func _ready():
	assert(radius != null)
	assert(height != null)

	#initialize collision
	var shape:CylinderShape = CylinderShape.new()
	shape.radius = radius
	shape.height = height
	collision_shape.shape = shape
	self.add_child(collision_shape)

	#initialize mesh
	var mesh:CylinderMesh = CylinderMesh.new()
	mesh.top_radius = radius
	mesh.bottom_radius = radius
	mesh.height = height
	var material:SpatialMaterial = SpatialMaterial.new()
	material.albedo_color = Color.white
	mesh.material = material
	mesh_instance.mesh = mesh
	self.add_child(mesh_instance)


func set_color(color:Color):
	self.mesh_instance.mesh.material.albedo_color = color


func set_height(value:float):
	collision_shape.shape.height = value
	mesh_instance.mesh.height = value

func set_radius(value:float):
	collision_shape.shape.radius = value
	mesh_instance.mesh.top_radius = value
	mesh_instance.mesh.bottom_radius = value

func place_on(location:Vector3):
	self.global_transform.origin = location + Vector3(0,height/2,0)
