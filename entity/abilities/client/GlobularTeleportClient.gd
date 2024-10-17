extends Spatial

class_name GlobularTeleportClient

onready var teleport_mesh_instance:MeshInstance = MeshInstance.new()

var vertices = []

var base: Vector3

func _ready():
	array_mesh_ready()

func spheres_ready():
	for point in vertices:
		var mesh_instance = MeshInstance.new()
		var mesh = SphereMesh.new()
		mesh.radius = 0.5
		mesh.height = 0.5
		mesh.material = SpatialMaterial.new()
		mesh.material.albedo_color = Color.purple
		mesh_instance.mesh = mesh
		mesh_instance.global_transform.origin = point
		self.add_child(mesh_instance)

	var base_instance = MeshInstance.new()
	var base_mesh = SphereMesh.new()
	base_mesh.radius = 10
	base_mesh.material = SpatialMaterial.new()
	base_mesh.material.albedo_color = Color.green
	base_instance.mesh = base_mesh
	base_instance.global_transform.origin = base
	self.add_child(base_instance)

func array_mesh_ready():
	self.global_transform.origin = base
	# Initialize the ArrayMesh.
	var arr_mesh = ArrayMesh.new()
	var arrays = []
	arrays.resize(ArrayMesh.ARRAY_MAX)
	arrays[ArrayMesh.ARRAY_VERTEX] = vertices
	# Create the Mesh.
	arr_mesh.add_surface_from_arrays(Mesh.PRIMITIVE_LINE_LOOP, arrays)
	#arr_mesh.material = SpatialMaterial.new()
	#arr_mesh.material.albedo_color = Color.purple
	teleport_mesh_instance.mesh = arr_mesh


	self.add_child(teleport_mesh_instance)

