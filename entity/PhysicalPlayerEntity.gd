extends Spatial

class_name PhysicalPlayerEntity
#export var body_instance:Resource
export var body_resource : Resource
onready var body = load(body_resource.resource_path).instance()
var id
func _ready():
	#body = load(body_instance.resource_path).instance()
	
	self.add_child(body)
	pass # Replace with function body.

func _physics_process(delta):
	
	self.global_transform.origin = body.global_transform.origin


func init_with_id(newId):
	id = newId
