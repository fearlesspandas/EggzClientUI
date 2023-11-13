extends Spatial


export var physical_entity:Resource
onready var entity
onready var message_controller = find_node("MessageController")
onready var physical_player_entity
#onready var physical_player_entity = find_node("PhysicalPlayerEntity")
export var id: String
var destinations = []
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
		{'ADDED_DESTINATION':{'id':physical_player_entity.body.id,'location':[var x, var y , var z]}}:
			print("destination added serverside")
			destinations.append(Vector3(x,y,z))
		_:
			pass
	pass
func _physics_process(delta):
	print("destinations:",destinations)
	ServerNetwork.setGlobLocation(id,physical_player_entity.body.global_transform.origin)
	ServerNetwork.get_next_destination(id)
	#produce physics events such as location change
	
	pass
func init_with_id(id):
	physical_player_entity.id = id
	id = id
