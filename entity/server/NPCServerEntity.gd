extends ServerEntity
class_name NPCServerEntity

func _ready():
	self.is_npc = true
	self.destinations_active = true
	self.timer.wait_time = 0.5
	self.add_child(timer)
	self.body.set_collision_layer_bit(EntityConstants.SERVER_PLAYER_COLLISION_LAYER,false)
	self.body.set_collision_mask_bit(EntityConstants.SERVER_PLAYER_COLLISION_LAYER,false)
	self.body.set_collision_layer_bit(EntityConstants.CLIENT_PLAYER_COLLISION_LAYER,false)
	self.body.set_collision_mask_bit(EntityConstants.CLIENT_PLAYER_COLLISION_LAYER,false)
	self.body.set_collision_layer_bit(EntityConstants.SERVER_NPC_COLLISION_LAYER,true)
	self.body.set_collision_mask_bit(EntityConstants.SERVER_NPC_COLLISION_LAYER,true)

	
