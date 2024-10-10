extends KinematicBody

class_name AxisArmServer

func _ready():
	self.set_collision_mask_bit(EntityConstants.SERVER_TERRAIN_COLLISION_LAYER,false)	
	self.set_collision_layer_bit(EntityConstants.SERVER_TERRAIN_COLLISION_LAYER,false)	
	self.set_collision_mask_bit(EntityConstants.SERVER_NPC_COLLISION_LAYER,true)	
	self.set_collision_layer_bit(EntityConstants.SERVER_NPC_COLLISION_LAYER,true)	
	#self.set_collision_mask_bit(EntityConstants.SERVER_BOSS_COLLISION_LAYER,true)	
	self.set_collision_layer_bit(EntityConstants.SERVER_PLAYER_COLLISION_LAYER,false)	
