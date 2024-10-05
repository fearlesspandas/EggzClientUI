extends KinematicBody

class_name ServerEntityKinematicBody
# Called when the node enters the scene tree for the first time.
onready var parent:ServerEntity = get_parent()
func _ready():
	pass

#todo move to ServerEntity?
func handle_ability_collision(ability_id:int):
	AbilityManager.do_ability_server(ability_id,parent.id)
