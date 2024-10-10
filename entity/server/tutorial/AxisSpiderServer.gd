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

	self.destinations_active = false

	self.gravity_active = false

	self.body.set_collision_layer_bit(EntityConstants.SERVER_TERRAIN_COLLISION_LAYER,false)
	self.body.set_collision_mask_bit(EntityConstants.SERVER_TERRAIN_COLLISION_LAYER,false)
	self.body.set_collision_mask_bit(EntityConstants.SERVER_PLAYER_COLLISION_LAYER,false)	
	self.body.set_collision_layer_bit(EntityConstants.SERVER_PLAYER_COLLISION_LAYER,false)	
	#self.body.set_collision_mask_bit(EntityConstants.SERVER_PLAYER_COLLISION_LAYER,false)
	#self.body.set_collision_layer_bit(EntityConstants.SERVER_PLAYER_COLLISION_LAYER,false)

	self.body.add_collision_exception_with(axis_core)
	self.body.add_collision_exception_with(axis_arm_1)
	self.body.add_collision_exception_with(axis_arm_2)
	self.body.add_collision_exception_with(axis_arm_3)
	self.body.add_collision_exception_with(axis_arm_4)
	self.body.add_collision_exception_with(axis_arm_5)
	self.body.add_collision_exception_with(axis_arm_6)
	self.body.add_collision_exception_with(axis_arm_7)
	self.body.add_collision_exception_with(axis_arm_8)


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
	#top_legs.global_rotation.y+= delta * rotation_speed
	#bottom_legs.global_rotation.y = -top_legs.global_rotation.y
	#physics_socket.set_rot_physics(id,top_legs.global_rotation)
	assert(!destinations_active)
	self.default_physics_process(delta)
	var collider = self.body.get_last_slide_collision()
	if collider != null:
		print_debug("SPIDER COLLISION ",collider.collider, " " , collider.collider_shape)
	#pass

func _handle_message(msg,delta_accum):
	self.default_handle_message(msg,delta_accum)
