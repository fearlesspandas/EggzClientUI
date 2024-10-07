extends NPCServerEntity

class_name AxisSpiderServer


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



func spider_physics_process(delta):
	physics_socket.get_dir_physics(id)
	update_lv_internal(axis_core,delta)
	movement.entity_set_max_speed(DataCache.cached(id,'speed'))
	movement.entity_move_by_direction(delta,axis_core)
	if !queued_teleports.empty() and body is KinematicBody and destinations_active:
		var t = queued_teleports.pop_front()
		var dir = (t - body.global_transform.origin)#.normalized()
		axis_core.translate(dir)
	if(!destination.is_empty and destinations_active):
		match destination.type:
			'{WAYPOINT:{}}':
				if gravity_active:
					movement.entity_move_by_gravity(id,delta,destination.location,axis_core)
				else:
					movement.entity_move(delta,destination.location,axis_core)
			'{TELEPORT:{}}':
				if gravity_active:
					movement.entity_move_by_gravity(id,delta,destination.location,axis_core)
				else:
					movement.entity_move(delta,destination.location,axis_core)
			"{GRAVITY_BIND:{}}":
				movement.entity_move_by_gravity(id,delta,destination.location,axis_core)
			_:
				print_debug("no handler found for destination with type ", axis_core)
	else:
		queued_teleports.pop_front()
	physics_socket.set_location_physics(id,axis_core.global_transform.origin)
	physics_socket.set_rot_physics(id,top_legs.global_rotation)
	
func _physics_process(delta):
	top_legs.global_rotation.y+= delta * rotation_speed
	bottom_legs.global_rotation.y = -top_legs.global_rotation.y
	physics_socket.set_rot_physics(id,top_legs.global_rotation)
	self.default_physics_process(delta)

func _handle_message(msg,delta_accum):
	self.default_handle_message(msg,delta_accum)
