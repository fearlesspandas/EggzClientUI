extends NonPlayerControlledEntity

class_name AxisSpiderClient

onready var arm_resource = load("res://entity/client/tutorial/AxisArmClient.tscn")
onready var core_resource = load("res://entity/server/tutorial/AxisCoreClient.tscn")
onready var top_legs = find_node("TopLegs")
onready var bottom_legs = find_node("BottomLegs")

#onready var axis_core = core_resource.instance()
#onready var axis_arm_1 = arm_resource.instance()
#onready var axis_arm_2 = arm_resource.instance()
#onready var axis_arm_3 = arm_resource.instance()
#onready var axis_arm_4 = arm_resource.instance()
#onready var axis_arm_5 = arm_resource.instance()
#onready var axis_arm_6 = arm_resource.instance()
#onready var axis_arm_7 = arm_resource.instance()
#onready var axis_arm_8 = arm_resource.instance()

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

	#self.add_child(axis_core)
	#axis_core.add_child(top_legs)
	#top_legs.add_child(axis_arm_1)
	#top_legs.add_child(axis_arm_2)
	#top_legs.add_child(axis_arm_3)
	#top_legs.add_child(axis_arm_4)

	#top_legs.add_child(axis_arm_5)
	#top_legs.add_child(axis_arm_6)
	#top_legs.add_child(axis_arm_7)
	#top_legs.add_child(axis_arm_8)

	#axis_arm_1.global_rotation += Vector3(0,0,0)
	#axis_arm_1.global_transform.origin += Vector3(100,0,0)
	#axis_arm_2.global_rotation += Vector3(0,0,0)
	#axis_arm_2.global_transform.origin += Vector3(-100,0,0)
	#axis_arm_3.global_rotation += Vector3(0,180,0)
	#axis_arm_3.global_transform.origin += Vector3(-100,90,0)
	#axis_arm_4.global_rotation += Vector3(0,0,0)
	#axis_arm_4.global_transform.origin += Vector3(100,-90,0)

	#axis_arm_5.global_rotation += Vector3(0,0,0)
	#axis_arm_5.global_transform.origin += Vector3(100,0,0)
	#axis_arm_6.global_rotation += Vector3(0,0,0)
	#axis_arm_6.global_transform.origin += Vector3(100,0,0)
	#axis_arm_7.global_rotation += Vector3(0,0,0)
	#axis_arm_7.global_transform.origin += Vector3(100,0,0)
	#axis_arm_8.global_rotation += Vector3(0,0,0)
	#axis_arm_8.global_transform.origin += Vector3(100,0,0)


func _physics_process(delta):
	physics_socket.get_rot_physics(id)
	self.default_physics_process(delta)

func _handle_message(msg,delta_accum):
	match msg:
		{'ROT':{'vec':[var x , var y ,var z]}}:
			pass
		_:
			self.default_handle_message(msg,delta_accum)
