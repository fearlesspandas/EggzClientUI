extends KinematicBody

#nodes that will 'extened' PhysicalPlayerEntity simply
export var mesh_instance:Resource
onready var mesh
func _ready():
	mesh = load(mesh_instance.resource_path).instance()
	
	self.add_child(mesh)
	self.set_collision_layer_bit(10,true)
	self.set_collision_mask_bit(10,true)
	pass # Replace with function body.
