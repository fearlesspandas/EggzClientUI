extends Spatial

class_name PlayerPathPointer

onready var sphere_mesh : SphereMesh = SphereMesh.new()
onready var pointer : MeshInstance = MeshInstance.new()
onready var material: SpatialMaterial = SpatialMaterial.new()


func _ready():
	sphere_mesh.radius = 1
	sphere_mesh.height = 1
	material.albedo_color = Color.red
	sphere_mesh.material = material
	pointer.mesh = sphere_mesh
	self.add_child(pointer)

func position(vector:Vector3):
	pointer.transform.origin = vector

	
