extends ServerEntity
class_name NPCServerEntity

func _ready():
	self.is_npc = true
	self.destinations_active = true
	self.timer.wait_time = 0.5
	self.add_child(timer)
	self.body.set_collision_layer_bit(10,false)
	self.body.set_collision_mask_bit(10,false)
	self.body.set_collision_layer_bit(11,false)
	self.body.set_collision_mask_bit(11,false)
	self.body.set_collision_layer_bit(12,false)
	self.body.set_collision_mask_bit(12,false)

	
