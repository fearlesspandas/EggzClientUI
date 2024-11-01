extends Spatial

class_name PlayerPathPointer

onready var sphere_mesh : SphereMesh = SphereMesh.new()
onready var pointer : MeshInstance = MeshInstance.new()
onready var material: SpatialMaterial = SpatialMaterial.new()
onready var POINTMESH:PointMesh = PointMesh.new()
onready var point : MeshInstance = MeshInstance.new()

onready var position_timer : Timer = Timer.new()
var dir:Vector3
func _ready():
	sphere_mesh.radius = 1
	sphere_mesh.height = 1
	material.albedo_color = Color.red
	sphere_mesh.material = material
	
	POINTMESH.material = SpatialMaterial.new()
	POINTMESH.material.flags_use_point_size = true
	POINTMESH.material.params_point_size = 30

	pointer.mesh = POINTMESH #= sphere_mesh
	self.add_child(pointer)

	position_timer.wait_time = 0.1
	position_timer.connect("timeout",self,"handle_position")
	self.add_child(position_timer) 
	position_timer.start()

func position(vector:Vector3):
	#pointer.transform.origin = vector
	dir = vector
	if vector.length() > 0:
		position_timer.set_paused(false)
	else:
		position_timer.set_paused(true)
		pointer.transform.origin = vector
	
func handle_position():
	pointer.transform.origin += dir
	
	
