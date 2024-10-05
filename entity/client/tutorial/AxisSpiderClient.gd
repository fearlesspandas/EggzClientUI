extends NonPlayerControlledEntity

class_name AxisSpiderClient

onready var arm_resource = load("res://entity/client/tutorial/AxisArmClient.tscn")
onready var core_resource = load("res://entity/server/tutorial/AxisCoreClient.tscn")
onready var top_legs:Spatial = Spatial.new()

onready var axis_core = core_resource.instance()
onready var axis_arm_1 = arm_resource.instance()
onready var axis_arm_2 = arm_resource.instance()
onready var axis_arm_3 = arm_resource.instance()
onready var axis_arm_4 = arm_resource.instance()
onready var axis_arm_5 = arm_resource.instance()
onready var axis_arm_6 = arm_resource.instance()
onready var axis_arm_7 = arm_resource.instance()
onready var axis_arm_8 = arm_resource.instance()

var rotation_speed:float = 0.1

func _ready():
	assert(top_legs != null)	
	assert(axis_core != null)
	assert(axis_arm_1 != null)
	assert(axis_arm_2 != null)
	assert(axis_arm_3 != null)
	assert(axis_arm_4 != null)
	assert(axis_arm_5 != null)
	assert(axis_arm_6 != null)
	assert(axis_arm_7 != null)
	assert(axis_arm_8 != null)

	self.add_child(axis_core)
	axis_core.add_child(top_legs)
	top_legs.add_child(axis_arm_1)
	top_legs.add_child(axis_arm_2)
	top_legs.add_child(axis_arm_3)
	top_legs.add_child(axis_arm_4)

	#top_legs.add_child(axis_arm_5)
	#top_legs.add_child(axis_arm_6)
	#top_legs.add_child(axis_arm_7)
	#top_legs.add_child(axis_arm_8)

	axis_arm_1.global_rotation += Vector3(0,0,0)
	axis_arm_1.global_transform.origin += Vector3(100,0,0)
	axis_arm_2.global_rotation += Vector3(0,0,0)
	axis_arm_2.global_transform.origin += Vector3(-100,0,0)
	axis_arm_3.global_rotation += Vector3(0,180,0)
	axis_arm_3.global_transform.origin += Vector3(-100,90,0)
	axis_arm_4.global_rotation += Vector3(0,0,0)
	axis_arm_4.global_transform.origin += Vector3(100,-90,0)

	#axis_arm_5.global_rotation += Vector3(0,0,0)
	#axis_arm_5.global_transform.origin += Vector3(100,0,0)
	#axis_arm_6.global_rotation += Vector3(0,0,0)
	#axis_arm_6.global_transform.origin += Vector3(100,0,0)
	#axis_arm_7.global_rotation += Vector3(0,0,0)
	#axis_arm_7.global_transform.origin += Vector3(100,0,0)
	#axis_arm_8.global_rotation += Vector3(0,0,0)
	#axis_arm_8.global_transform.origin += Vector3(100,0,0)


func _physics_process(delta):
	physics_socket.get_location_physics(id)
	physics_socket.get_rot_physics(id)


