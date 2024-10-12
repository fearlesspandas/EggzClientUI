extends NPCServerEntity

class_name AxisSpiderServer


onready var top_legs = body.find_node("TopLegs")
onready var bottom_legs = body.find_node("BottomLegs")

onready var axis_core = body.find_node("AxisCore")
onready var axis_arm_1 = body.find_node("AxisArm1")
onready var axis_arm_2 = body.find_node("AxisArm2")
onready var axis_arm_3 = body.find_node("AxisArm3")
onready var axis_arm_4 = body.find_node("AxisArm4")
onready var axis_arm_5 = body.find_node("AxisArm5")
onready var axis_arm_6 = body.find_node("AxisArm6")
onready var axis_arm_7 = body.find_node("AxisArm7")
onready var axis_arm_8 = body.find_node("AxisArm8")

onready var setup_timer : Timer = Timer.new()

var path = [] 

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
	self.body.set_collision_mask_bit(EntityConstants.SERVER_PLAYER_COLLISION_LAYER,false)	
	self.body.set_collision_layer_bit(EntityConstants.SERVER_PLAYER_COLLISION_LAYER,false)	
	#self.body.set_collision_mask_bit(EntityConstants.SERVER_PLAYER_COLLISION_LAYER,false)
	#self.body.set_collision_layer_bit(EntityConstants.SERVER_PLAYER_COLLISION_LAYER,false)

	#self.body.add_collision_exception_with(axis_core)
	#self.body.add_collision_exception_with(axis_arm_1)
	#self.body.add_collision_exception_with(axis_arm_2)
	#self.body.add_collision_exception_with(axis_arm_3)
	#self.body.add_collision_exception_with(axis_arm_4)
	#self.body.add_collision_exception_with(axis_arm_5)
	#self.body.add_collision_exception_with(axis_arm_6)
	#self.body.add_collision_exception_with(axis_arm_7)
	#self.body.add_collision_exception_with(axis_arm_8)

	setup_timer.wait_time = 0.5
	setup_timer.connect("timeout",self,"setup")
	self.add_child(setup_timer)
	setup_timer.start()

	

func setup_path():
	socket.add_destination(id,self.global_transform.origin + Vector3(500,250,0),"WAYPOINT",1)
	socket.add_destination(id,self.global_transform.origin + Vector3(-500,250,0),"WAYPOINT",1)
	socket.add_destination(id,self.global_transform.origin + Vector3(500,-250,0),"WAYPOINT",1)
	socket.add_destination(id,self.global_transform.origin + Vector3(-500,-250,0),"WAYPOINT",1)
	socket.add_destination(id,self.global_transform.origin + Vector3(0,250,500),"WAYPOINT",1)
	socket.add_destination(id,self.global_transform.origin + Vector3(0,250,-500),"WAYPOINT",1)
	socket.add_destination(id,self.global_transform.origin + Vector3(0,-250,500),"WAYPOINT",1)
	socket.add_destination(id,self.global_transform.origin + Vector3(0,-250,-500),"WAYPOINT",1)

	

func setup():
	setup_timer.one_shot = true
	setup_timer.stop()
	#reset physics
	physics_socket.set_dir_physics(id,Vector3.ZERO)
	socket.adjust_max_speed(id,500)
	socket.set_speed(id,300)
	#reset destinations
	socket.set_destination_mode(id,"FORWARD")
	socket.clear_destinations(id)
	socket.set_gravitate(id,false)
	setup_path()

func _physics_process(delta):
	top_legs.global_rotation.y+= delta * rotation_speed
	bottom_legs.global_rotation.y = -top_legs.global_rotation.y
	physics_socket.set_rot_physics(id,top_legs.global_rotation)
	self.default_physics_process(delta)

func _handle_message(msg,delta_accum):
	self.default_handle_message(msg,delta_accum)
