extends KinematicBody

#nodes that will 'extened' PhysicalPlayerEntity simply
export var mesh_instance:Resource
onready var mesh
func _ready():
	mesh = load(mesh_instance.resource_path).instance()
	
	self.add_child(mesh)
	pass # Replace with function body.
