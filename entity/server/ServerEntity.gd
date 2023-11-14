extends PhysicalPlayerEntity
class_name ServerEntity

onready var message_controller:MessageController = MessageController.new()
onready var movement:ServerKinematicMovement = ServerKinematicMovement.new()
onready var spawn
var destinations = []
var requested_dest = false
var timeout = 100
var last_request = null

func _ready():
	spawn = body.global_transform.origin
	self.add_child(message_controller)
	pass # Replace with function body.

func _handle_message(msg,delta_accum):
	#handle server messages, starting with movement
	#most of these will be actions (add new destination, change velocity etc..)
	print("entered server message handler")
	match msg:
		{'SET_GLOB_LOCATION':{'id':id,'location':var location}}:
			body.global_transform.origin = location
		{"NextDestination":{"id": var id, "location": [var x, var y , var z]}}:
			print("destination added serverside", [x,y,z])
			destinations.append(Vector3(x,y,z))
			requested_dest = false
		_:
			print("No server entity handler for " , msg)
			pass
	pass
func freeze():
	body.global_transform.origin = spawn
func _physics_process(delta):
	#freeze()
	ServerNetwork.setGlobLocation(id,body.global_transform.origin)
	
	if requested_dest and last_request != null and OS.get_ticks_msec() - last_request > timeout:
		requested_dest = false
	if destinations.size() == 0 and !requested_dest:
		print("requesting destination")
		ServerNetwork.get_next_destination(id)
		requested_dest = true
		last_request  = OS.get_ticks_msec()
	elif destinations.size() == 0 and requested_dest:
		pass
		#print("waiting for dest response")
	else:
		var dest = destinations.pop_front()
		movement.move(delta,dest,body)
		print("popping destination",dest)
		
	#produce physics events such as location change
	
	pass
