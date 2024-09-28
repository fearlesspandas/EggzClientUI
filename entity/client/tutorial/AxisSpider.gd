extends Spatial

class_name AxisSpider

onready var top_legs = find_node("TopLegs")
onready var bottom_legs = find_node("BottomLegs")
onready var axis_core = find_node("AxisCore")
onready var axis_arm_1 = find_node("AxisArm1")
onready var axis_arm_2 = find_node("AxisArm2")
onready var axis_arm_3 = find_node("AxisArm3")
onready var axis_arm_4 = find_node("AxisArm4")
onready var axis_arm_5 = find_node("AxisArm5")
onready var axis_arm_6 = find_node("AxisArm6")
onready var axis_arm_7 = find_node("AxisArm7")
onready var axis_arm_8 = find_node("AxisArm8")

var rotation_speed:float = 0.1

func _ready():
	assert(top_legs != null)	
	assert(bottom_legs != null)
	assert(axis_core != null)
	assert(axis_arm_1 != null)
	assert(axis_arm_2 != null)
	assert(axis_arm_3 != null)
	assert(axis_arm_4 != null)
	assert(axis_arm_5 != null)
	assert(axis_arm_6 != null)
	assert(axis_arm_7 != null)
	assert(axis_arm_8 != null)

	axis_core.find_node("KinematicBody").set_collision_mask_bit(EntityConstants.SERVER_TERRAIN_COLLISION_LAYER,false)	
	axis_core.find_node("KinematicBody").set_collision_layer_bit(EntityConstants.SERVER_TERRAIN_COLLISION_LAYER,false)	

	axis_arm_1.find_node("KinematicBody").set_collision_mask_bit(EntityConstants.SERVER_TERRAIN_COLLISION_LAYER,false)	
	axis_arm_1.find_node("KinematicBody").set_collision_layer_bit(EntityConstants.SERVER_TERRAIN_COLLISION_LAYER,false)	

	axis_arm_2.find_node("KinematicBody").set_collision_mask_bit(EntityConstants.SERVER_TERRAIN_COLLISION_LAYER,false)	
	axis_arm_2.find_node("KinematicBody").set_collision_layer_bit(EntityConstants.SERVER_TERRAIN_COLLISION_LAYER,false)	

	axis_arm_3.find_node("KinematicBody").set_collision_mask_bit(EntityConstants.SERVER_TERRAIN_COLLISION_LAYER,false)	
	axis_arm_3.find_node("KinematicBody").set_collision_layer_bit(EntityConstants.SERVER_TERRAIN_COLLISION_LAYER,false)	

	axis_arm_4.find_node("KinematicBody").set_collision_mask_bit(EntityConstants.SERVER_TERRAIN_COLLISION_LAYER,false)	
	axis_arm_4.find_node("KinematicBody").set_collision_layer_bit(EntityConstants.SERVER_TERRAIN_COLLISION_LAYER,false)	

	axis_arm_5.find_node("KinematicBody").set_collision_mask_bit(EntityConstants.SERVER_TERRAIN_COLLISION_LAYER,false)	
	axis_arm_5.find_node("KinematicBody").set_collision_layer_bit(EntityConstants.SERVER_TERRAIN_COLLISION_LAYER,false)	

	axis_arm_6.find_node("KinematicBody").set_collision_mask_bit(EntityConstants.SERVER_TERRAIN_COLLISION_LAYER,false)	
	axis_arm_6.find_node("KinematicBody").set_collision_layer_bit(EntityConstants.SERVER_TERRAIN_COLLISION_LAYER,false)	

	axis_arm_7.find_node("KinematicBody").set_collision_mask_bit(EntityConstants.SERVER_TERRAIN_COLLISION_LAYER,false)	
	axis_arm_7.find_node("KinematicBody").set_collision_layer_bit(EntityConstants.SERVER_TERRAIN_COLLISION_LAYER,false)	

	axis_arm_8.find_node("KinematicBody").set_collision_mask_bit(EntityConstants.SERVER_TERRAIN_COLLISION_LAYER,false)	
	axis_arm_8.find_node("KinematicBody").set_collision_layer_bit(EntityConstants.SERVER_TERRAIN_COLLISION_LAYER,false)	

	axis_core.find_node("KinematicBody").set_collision_mask_bit(EntityConstants.CLIENT_NPC_COLLISION_LAYER,true)	
	axis_core.find_node("KinematicBody").set_collision_layer_bit(EntityConstants.CLIENT_NPC_COLLISION_LAYER,true)	

	axis_arm_1.find_node("KinematicBody").set_collision_mask_bit(EntityConstants.CLIENT_NPC_COLLISION_LAYER,true)	
	axis_arm_1.find_node("KinematicBody").set_collision_layer_bit(EntityConstants.CLIENT_NPC_COLLISION_LAYER,true)	

	axis_arm_2.find_node("KinematicBody").set_collision_mask_bit(EntityConstants.CLIENT_NPC_COLLISION_LAYER,true)	
	axis_arm_2.find_node("KinematicBody").set_collision_layer_bit(EntityConstants.CLIENT_NPC_COLLISION_LAYER,true)	

	axis_arm_3.find_node("KinematicBody").set_collision_mask_bit(EntityConstants.CLIENT_NPC_COLLISION_LAYER,true)	
	axis_arm_3.find_node("KinematicBody").set_collision_layer_bit(EntityConstants.CLIENT_NPC_COLLISION_LAYER,true)	

	axis_arm_4.find_node("KinematicBody").set_collision_mask_bit(EntityConstants.CLIENT_NPC_COLLISION_LAYER,true)	
	axis_arm_4.find_node("KinematicBody").set_collision_layer_bit(EntityConstants.CLIENT_NPC_COLLISION_LAYER,true)	

	axis_arm_5.find_node("KinematicBody").set_collision_mask_bit(EntityConstants.CLIENT_NPC_COLLISION_LAYER,true)	
	axis_arm_5.find_node("KinematicBody").set_collision_layer_bit(EntityConstants.CLIENT_NPC_COLLISION_LAYER,true)	

	axis_arm_6.find_node("KinematicBody").set_collision_mask_bit(EntityConstants.CLIENT_NPC_COLLISION_LAYER,true)	
	axis_arm_6.find_node("KinematicBody").set_collision_layer_bit(EntityConstants.CLIENT_NPC_COLLISION_LAYER,true)	

	axis_arm_7.find_node("KinematicBody").set_collision_mask_bit(EntityConstants.CLIENT_NPC_COLLISION_LAYER,true)	
	axis_arm_7.find_node("KinematicBody").set_collision_layer_bit(EntityConstants.CLIENT_NPC_COLLISION_LAYER,true)	

	axis_arm_8.find_node("KinematicBody").set_collision_mask_bit(EntityConstants.CLIENT_NPC_COLLISION_LAYER,true)	
	axis_arm_8.find_node("KinematicBody").set_collision_layer_bit(EntityConstants.CLIENT_NPC_COLLISION_LAYER,true)	



	
func _physics_process(delta):
	top_legs.global_rotation.y+= delta * rotation_speed
	bottom_legs.global_rotation.y -= delta * rotation_speed	
