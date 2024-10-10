extends NonPlayerControlledEntity
class_name AxisSpiderClient

onready var arm_resource = load("res://entity/client/tutorial/AxisArmClient.tscn")
onready var core_resource = load("res://entity/server/tutorial/AxisCoreClient.tscn")

onready var top_legs = body.find_node("TopLegs")
onready var bottom_legs = body.find_node("BottomLegs")

#onready var axis_core = core_resource.instance()
#onready var axis_arm_1 = arm_resource.instance()
#onready var axis_arm_2 = arm_resource.instance()
#onready var axis_arm_3 = arm_resource.instance()
#onready var axis_arm_4 = arm_resource.instance()
#onready var axis_arm_5 = arm_resource.instance()
#onready var axis_arm_6 = arm_resource.instance()
#onready var axis_arm_7 = arm_resource.instance()
#onready var axis_arm_8 = arm_resource.instance()

onready var axis_core = body.find_node("AxisCore")
onready var axis_arm_1 = body.find_node("AxisArm1")
onready var axis_arm_2 = body.find_node("AxisArm2")
onready var axis_arm_3 = body.find_node("AxisArm3")
onready var axis_arm_4 = body.find_node("AxisArm4")
onready var axis_arm_5 = body.find_node("AxisArm5")
onready var axis_arm_6 = body.find_node("AxisArm6")
onready var axis_arm_7 = body.find_node("AxisArm7")
onready var axis_arm_8 = body.find_node("AxisArm8")
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
	self.body.set_collision_layer_bit(EntityConstants.SERVER_TERRAIN_COLLISION_LAYER,false)
	self.body.set_collision_mask_bit(EntityConstants.SERVER_TERRAIN_COLLISION_LAYER,false)
	self.body.set_collision_layer_bit(EntityConstants.SERVER_NPC_COLLISION_LAYER,false)
	self.body.set_collision_mask_bit(EntityConstants.SERVER_NPC_COLLISION_LAYER,false)
	self.body.add_collision_exception_with(axis_core)
	self.body.add_collision_exception_with(axis_arm_1)
	self.body.add_collision_exception_with(axis_arm_2)
	self.body.add_collision_exception_with(axis_arm_3)
	self.body.add_collision_exception_with(axis_arm_4)
	self.body.add_collision_exception_with(axis_arm_5)
	self.body.add_collision_exception_with(axis_arm_6)
	self.body.add_collision_exception_with(axis_arm_7)
	self.body.add_collision_exception_with(axis_arm_8)


func _physics_process(delta):
	physics_socket.get_rot_physics(id)
	self.default_physics_process(delta)

func _handle_message(msg,delta_accum):
	match msg:
		{'Rot':{'id':var id,'vec':[var x , var y ,var z]}}:
			top_legs.global_rotation.y = y #Vector3(x,y,z)
			bottom_legs.global_rotation.y = -y #Vector3(x,y,z)
		_:
			self.default_handle_message(msg,delta_accum)
