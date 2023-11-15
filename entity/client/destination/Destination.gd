extends Spatial

class_name Destination

export var mesh:Resource #mesh instance for destination
onready var model = load(mesh.resource_path).instance()
func _ready():
	self.add_child(model)
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
#func _process(delta):
#	pass
