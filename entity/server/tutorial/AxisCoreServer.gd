extends KinematicBody

class_name AxisCoreServer

func _ready():
	self.set_collision_mask_bit(EntityConstants.SERVER_TERRAIN_COLLISION_LAYER,false)	
	self.set_collision_layer_bit(EntityConstants.SERVER_TERRAIN_COLLISION_LAYER,false)	
	self.set_collision_mask_bit(EntityConstants.SERVER_NPC_COLLISION_LAYER,true)	
	self.set_collision_layer_bit(EntityConstants.SERVER_NPC_COLLISION_LAYER,true)	
