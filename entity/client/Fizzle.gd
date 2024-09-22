extends Spatial
class_name Fizzle

onready var top:MeshInstance = MeshInstance.new()
onready var bottom:MeshInstance = MeshInstance.new()
onready var left:MeshInstance = MeshInstance.new()
onready var right:MeshInstance = MeshInstance.new()
onready var forward:MeshInstance = MeshInstance.new()
onready var back:MeshInstance = MeshInstance.new()

var center:Vector3

func _ready():
	assert(center != null and center != Vector3())
	self.global_transform.origin = center

	var spheremesh:SphereMesh = SphereMesh.new()
	spheremesh.radius = 0.5
	top.mesh = spheremesh
	bottom.mesh = spheremesh
	left.mesh = spheremesh
	right.mesh = spheremesh
	forward.mesh = spheremesh
	back.mesh = spheremesh

	self.add_child(top)
	self.add_child(bottom)
	self.add_child(left)
	self.add_child(right)
	self.add_child(forward)
	self.add_child(back)

	top.global_transform.origin = self.global_transform.origin + (Vector3(0,1,0) * 3)
	bottom.global_transform.origin = self.global_transform.origin + (Vector3(0,-1,0) * 3)
	left.global_transform.origin = self.global_transform.origin + (Vector3(-1,0,0) * 3)
	right.global_transform.origin = self.global_transform.origin + (Vector3(1,0,0) * 3)
	forward.global_transform.origin = self.global_transform.origin + (Vector3(0,0,1) * 3)
	back.global_transform.origin = self.global_transform.origin + (Vector3(0,0,-1) * 3)
