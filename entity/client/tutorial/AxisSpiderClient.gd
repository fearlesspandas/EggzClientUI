extends NonPlayerControlledEntity
class_name AxisSpiderClient

onready var arm_resource = load("res://entity/client/tutorial/AxisArmClient.tscn")
onready var core_resource = load("res://entity/server/tutorial/AxisCoreClient.tscn")

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

var rotation_speed:float = 0.1
#var dest_positions = [
#	self.global_transform.origin + Vector3(500,250,0),
#	self.global_transform.origin + Vector3(-500,250,0),
#	self.global_transform.origin + Vector3(500,-250,0),
#	self.global_transform.origin + Vector3(-500,-250,0),
#	self.global_transform.origin + Vector3(0,250,500),
#	self.global_transform.origin + Vector3(0,250,-500),
#	self.global_transform.origin + Vector3(0,-250,500),
#	self.global_transform.origin + Vector3(0,-250,-500),
#]

func set_visibility(is_visible:bool):
	top_legs.visible = is_visible
	bottom_legs.visible = is_visible
	axis_core.visible = is_visible
	axis_arm_1.visible = is_visible
	axis_arm_2.visible = is_visible
	axis_arm_3.visible = is_visible
	axis_arm_4.visible = is_visible
	axis_arm_5.visible = is_visible
	axis_arm_6.visible = is_visible
	axis_arm_7.visible = is_visible
	axis_arm_8.visible = is_visible

func _ready():
	self.radius = 500
	self.mod = 2
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
	#self.body.add_collision_exception_with(axis_core)
	#self.body.add_collision_exception_with(axis_arm_1)
	#self.body.add_collision_exception_with(axis_arm_2)
	#self.body.add_collision_exception_with(axis_arm_3)
	#self.body.add_collision_exception_with(axis_arm_4)
	#self.body.add_collision_exception_with(axis_arm_5)
	#self.body.add_collision_exception_with(axis_arm_6)
	#self.body.add_collision_exception_with(axis_arm_7)
	#self.body.add_collision_exception_with(axis_arm_8)

	set_visibility(false)

	setup_timer.wait_time = 0.5
	setup_timer.connect("timeout",self,"setup")
	self.add_child(setup_timer)
	setup_timer.start()

	GlobalSignalsClient.connect("player_position",self,"default_update_player_location")
	#will have to remove this when a generic spider entity is made
	ProgressHandlerClient.connect("tutorial_stage_completed",self,"tutorial_stage_completed")
	physics_native_socket = load("res://native_lib/ClientPhysicsSocket.gdns").new()

func tutorial_stage_completed(stage:int):
	set_visibility(true)

func destination_mesh() -> MeshInstance:
	var mesh:CubeMesh = CubeMesh.new()
	mesh.size = Vector3(20,20,20)
	mesh.material = SpatialMaterial.new()
	mesh.material.albedo_color = Color.purple
	var res = MeshInstance.new()
	res.mesh = mesh
	return res
	
func destination_mesh_at(location:Vector3) -> MeshInstance:
	var res = destination_mesh()
	res.global_transform.origin = location
	return res

func setup_path():
	self.add_child(destination_mesh_at(self.global_transform.origin + Vector3(500,250,0)))
	self.add_child(destination_mesh_at(self.global_transform.origin + Vector3(-500,250,0)))
	self.add_child(destination_mesh_at(self.global_transform.origin + Vector3(500,-250,0)))
	self.add_child(destination_mesh_at(self.global_transform.origin + Vector3(-500,-250,0)))
	self.add_child(destination_mesh_at(self.global_transform.origin + Vector3(0,250,500)))
	self.add_child(destination_mesh_at(self.global_transform.origin + Vector3(0,250,-500)))
	self.add_child(destination_mesh_at(self.global_transform.origin + Vector3(0,-250,500)))
	self.add_child(destination_mesh_at(self.global_transform.origin + Vector3(0,-250,-500)))
	
func setup():
	setup_timer.one_shot = true
	setup_timer.stop()
	pass
#	setup_path()

func _physics_process(delta):
	physics_native_socket.get_rotation(id)
	var rot = physics_native_socket.cached_rotation()
	if rot != null:
		top_legs.global_rotation.y = rot[1]
		bottom_legs.global_rotation.y = -rot[1]
		self.default_physics_process(delta,self.mod)

func _handle_message(msg,delta_accum):
	match msg:
		{'typ':var typ,'id':var id,'vec' : [var x , var y , var z]}:
			match typ:
				'Rot':
					top_legs.global_rotation.y = y
					bottom_legs.global_rotation.y = -y
				_:
					default_handle_message(msg,delta_accum)
		_:
			self.default_handle_message(msg,delta_accum)
