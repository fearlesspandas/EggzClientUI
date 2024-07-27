extends Spatial

class_name PhysicalPlayerEntity
#export var body_instance:Resource
export var body_resource : Resource
export var movement_resource:Resource
onready var body = load(body_resource.resource_path).instance()
onready var movement:Movement = load(movement_resource.resource_path).new()
var id
var client_id
func _ready():
	#body = load(body_instance.resource_path).instance()
	self.add_child(movement)
	self.add_child(body)
	pass # Replace with function body.

func _physics_process(delta):
	self.global_transform.origin = body.global_transform.origin

func swap_body(resource:Resource):
	var nwbody = load(resource.resource_path).instance()
	self.remove_child(body)
	body.call_deferred('free')
	self.add_child(nwbody)
	body = nwbody
	
func swap_movement(resource:Resource):
	var nwmv = load(resource.resource_path).new()
	self.remove_child(movement)
	movement.call_deferred('free')
	self.add_child(nwmv)
	movement = nwmv

func swap_body_and_movement(body_rsc:Resource,mv_rsc:Resource):
	var nwbody = load(body_rsc.resource_path).instance()
	var nwmv = load(mv_rsc.resource_path).new()
	self.remove_child(movement)
	self.remove_child(body)
	body.call_deferred('free')
	movement.call_deferred('free')
	self.add_child(nwbody)
	self.add_child(nwmv)
	body = nwbody
	movement = nwmv
	
func init_with_id(newId,clientID):
	id = newId
	client_id = clientID
