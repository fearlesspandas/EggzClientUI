extends Node


export var physical_entity:Resource
onready var entity
onready var message_controller = find_node("Message_controller")
onready var physical_player_entity = entity.find_node("PhysicalPlayerEntity")
export var id: String

func _ready():
	entity = load(physical_entity.resource_path).instance()
	self.add_child(entity)
	pass # Replace with function body.

func _handle_message(msg,delta_accum):
	#handle server messages, starting with movement
	#most of these will be actions (add new destination, change velocity etc..)
	pass
func _physics_process(delta):
	#produce physics events such as location change
	ServerNetwork.setGlobLocation(id,physical_player_entity.body.global_transform.origin)
	pass
