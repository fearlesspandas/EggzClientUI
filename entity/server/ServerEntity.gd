extends PhysicalPlayerEntity
class_name ServerEntity

onready var message_controller:MessageController = MessageController.new()
onready var spawn
var requested_dest = false
var timeout = 10
var last_request = null
var destination=null
var epsilon = 3
func _ready():
	spawn = body.global_transform.origin
	self.add_child(message_controller)
	pass # Replace with function body.

func _handle_message(msg,delta_accum):
	#handle server messages, starting with movement
	#most of these will be actions (add new destination, change velocity etc..)
	match msg:
		{'SET_GLOB_LOCATION':{'id':id,'location':var location}}:
			body.global_transform.origin = location
		{"NextDestination":{"id": var id, "location": [var x, var y , var z]}}:
			#print("destination added serverside", [x,y,z])
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
	#freeze()
	self.global_transform.origin = body.global_transform.origin
	var socket = ServerNetwork.get(client_id,false)
	if socket != null:
		socket.setGlobLocation(id,body.global_transform.origin)
		socket.get_next_destination(id)
	if(destination != null ):
		var diff = destination - body.global_transform.origin
		if diff.length() > epsilon:	
			movement.entity_move(delta,destination,body)
			#print("active destination",destination)
		else:
			destination = null
		
	#produce physics events such as location change
	
	pass
