extends Spatial

class_name SmackClient

onready var mesh_instance:MeshInstance = MeshInstance.new()
onready var despawn_timer:Timer = Timer.new()

func _ready():
	var mesh:SphereMesh = SphereMesh.new()
	mesh.radius = 10
	mesh.height = 10
	
	var material:SpatialMaterial = SpatialMaterial.new()
	material.albedo_color = Color.yellow
	
	mesh.material = material	
	mesh_instance.mesh = mesh

	self.add_child(mesh_instance)

	#timer setup
	despawn_timer.wait_time = 1 
	despawn_timer.connect("timeout",self,"despawn")
	self.add_child(despawn_timer)
	despawn_timer.start()

func despawn():
	self.remove_child(mesh_instance)
	mesh_instance.call_deferred("free")
	self.remove_child(despawn_timer)
	despawn_timer.call_deferred("free")
	get_parent().remove_child(self)
	self.call_deferred("free")
