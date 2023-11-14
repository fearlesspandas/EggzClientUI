extends Spatial


export var physical_entity:Resource
onready var entity
onready var message_controller = find_node("MessageController")
onready var physical_player_entity
#onready var physical_player_entity = find_node("PhysicalPlayerEntity")
export var id: String
var destinations = []
var requested_dest = false
var timeout = 100
var last_request = null
func _ready():
	entity = load(physical_entity.resource_path).instance()
	physical_player_entity = entity.find_node("PhysicalPlayerEntity")
	self.add_child(entity)
	pass # Replace with function body.

func _handle_message(msg,delta_accum):
	#handle server messages, starting with movement
	#most of these will be actions (add new destination, change velocity etc..)
	print("entered server message handler")
	match msg:
		{'SET_GLOB_LOCATION':{'id':physical_player_entity.id,'location':var location}}:
			physical_player_entity.body.global_transform.origin = location
		{"NextDestination":{"id": var id, "location": [var x, var y , var z]}}:
			print("destination added serverside", [x,y,z])
			destinations.append(Vector3(x,y,z))
			requested_dest = false
		_:
			print("No server entity handler for " , msg)
			pass
	pass
func _physics_process(delta):
	ServerNetwork.setGlobLocation(id,physical_player_entity.body.global_transform.origin)
	
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
		print("popping destination",destinations.pop_front())
	#produce physics events such as location change
	
	pass
func init_with_id(id):
	physical_player_entity.id = id
	id = id
