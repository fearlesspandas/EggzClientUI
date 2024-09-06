extends ClientPlayerEntity
class_name NonPlayerControlledEntity
func _physics_process(delta):
	pass
	

func _ready():
	self.is_npc = true
	self.mod = 8
