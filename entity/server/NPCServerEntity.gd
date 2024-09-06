extends ServerEntity
class_name NPCServerEntity

func _ready():
	self.is_npc = true
	self.destinations_active = true
	self.timer.wait_time = 0.5
	self.add_child(timer)

	
