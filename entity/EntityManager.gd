extends Node



var entities = {}
func create_entity(id:String,resource:Resource):
	var res = load(resource.resource_path).instance()
	self.add_child(res)
	entities[id] = res
func _ready():
	pass # Replace with function body.
