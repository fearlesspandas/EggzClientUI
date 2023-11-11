extends Node



export var mesh_instance:Resource
export var physical_entity:Resource
func _ready():
	var mesh = load(mesh_instance.resource_path).instance()
	var pe = load(physical_entity.resource_path).instance()
	self.add_child(mesh)
	self.add_child(pe)
	pass # Replace with function body.
