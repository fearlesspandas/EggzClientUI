extends PhysicalPlayerEntity
class_name ServerEntity

onready var message_controller:MessageController = MessageController.new()
onready var spawn
var requested_dest = false
var timeout = 10
var last_request = null
var destination=null
var epsilon = 3
var isSubbed:bool = false
func _ready():
	spawn = body.global_transform.origin
	self.add_child(message_controller)
	pass # Replace with function body.

func _handle_message(msg,delta_accum):
	match msg:
		{'Input':{"id":var id, "vec":[var x ,var y ,var z]}}:
			movement.entity_apply_vector(delta_accum,Vector3(x,y,z),body)
		{'SET_GLOB_LOCATION':{'id':id,'location':var location}}:
			body.global_transform.origin = location
		{"NextDestination":{"id": var id, "location": [var x, var y , var z]}}:
			requested_dest = false
			destination = Vector3(x,y,z)
			#print("set destination successfully")
		_:
			print("No server entity handler for " , msg)
			pass
	pass
	
func freeze():
	body.global_transform.origin = spawn
	
func _physics_process(delta):
	#print("server entity location", body.global_transform.origin)
	self.global_transform.origin = body.global_transform.origin
	var socket = ServerNetwork.get(client_id)
	if socket != null:
		socket.setGlobLocation(id,body.global_transform.origin)
		socket.get_next_destination(id)
	if(destination != null ):
		var diff = destination - body.global_transform.origin
		if diff.length() > epsilon:	
			movement.entity_move(delta,destination,body)
			#print("active destination",destination)
		else:
			movement.entity_stop(delta,body)
			destination = null
	pass
	
func _process(delta):
	var socket = ServerNetwork.get(client_id)
	if !isSubbed and socket != null :
		socket.input_subscribe(id)
		isSubbed = true
		print("server entity, subbing to input for id", id)
		
func _input(event):
	if  event.is_action_pressed("control"):
		print("stopping!!")
		movement.entity_stop(0,body)
