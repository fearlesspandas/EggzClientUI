extends KinematicBody
class_name ClientEntityKinematicBody

onready var client_player_entity:ClientPlayerEntity = get_parent()

func _ready():
	assert(client_player_entity != null)
