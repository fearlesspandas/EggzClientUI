extends Spatial

class_name AxisArmServer

onready var vertical = find_node("ArmVertical") 
onready var horizontal = find_node("ArmHorizontal") 
onready var elbow = find_node("Elbow") 
func _ready():
	vertical.set_collision_mask_bit(EntityConstants.SERVER_TERRAIN_COLLISION_LAYER,false)	
	vertical.set_collision_layer_bit(EntityConstants.SERVER_TERRAIN_COLLISION_LAYER,false)	
	#self.set_collision_mask_bit(EntityConstants.SERVER_NPC_COLLISION_LAYER,true)	
	vertical.set_collision_layer_bit(EntityConstants.SERVER_NPC_COLLISION_LAYER,true)	
	#self.set_collision_mask_bit(EntityConstants.SERVER_BOSS_COLLISION_LAYER,true)	
	vertical.set_collision_layer_bit(EntityConstants.SERVER_PLAYER_COLLISION_LAYER,false)	
	

	horizontal.set_collision_mask_bit(EntityConstants.SERVER_TERRAIN_COLLISION_LAYER,false)	
	horizontal.set_collision_layer_bit(EntityConstants.SERVER_TERRAIN_COLLISION_LAYER,false)	
	horizontal.set_collision_layer_bit(EntityConstants.SERVER_NPC_COLLISION_LAYER,true)	
	horizontal.set_collision_layer_bit(EntityConstants.SERVER_PLAYER_COLLISION_LAYER,false)	


	elbow.set_collision_mask_bit(EntityConstants.SERVER_TERRAIN_COLLISION_LAYER,false)	
	elbow.set_collision_layer_bit(EntityConstants.SERVER_TERRAIN_COLLISION_LAYER,false)	
	elbow.set_collision_layer_bit(EntityConstants.SERVER_NPC_COLLISION_LAYER,true)	
	elbow.set_collision_layer_bit(EntityConstants.SERVER_PLAYER_COLLISION_LAYER,false)	
