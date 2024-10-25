extends ClientPlayerEntity

class_name Player
signal set_destination_mode(mode)
signal set_gravity_active(is_active)
signal set_destinations_active(is_active)

onready var camera_root =find_node("CameraRoot")
onready var camera:Camera = camera_root.find_node("Camera")
onready var curserRay:CursorRay = camera_root.find_node("CursorRay")
onready var pointer:PlayerPathPointer = PlayerPathPointer.new()
onready var input_timer:Timer = Timer.new()
onready var position_data_timer : Timer = Timer.new()
onready var navigator_mesh:NavigatorMesh = NavigatorMesh.new()

var is_active = false

func _ready():
	self.is_npc = false
	body.add_child(pointer)
	physics_socket = ServerNetwork.get_physics(client_id)
	assert(physics_socket!=null)
	input_timer.wait_time = 0.1
	input_timer.connect("timeout",self,"muh_process")
	body.add_child(navigator_mesh)
	
func set_destination_mode(mode):
	var mode_text = str(mode).replace("{","").replace("}","").replace(":","")
	emit_signal("set_destination_mode",mode_text)
	
func set_gravity_active(is_active):
	emit_signal("set_gravity_active",is_active)
	
func set_destinations_active(is_active):
	emit_signal("set_destinations_active",is_active)
	
#todo move destination logic to destination manager
func _input(event):
	if is_active and event is InputEventKey and event.is_action_released("reverse_queue_destinations"):
		var socket = ServerNetwork.get(client_id)
		socket.set_destination_mode(id,"REVERSE")
		emit_signal("set_destination_mode","REVERSE")
	if is_active and event is InputEventKey and event.is_action_released("pop_destinations"):
		var socket = ServerNetwork.get(client_id)
		socket.set_destination_mode(id,"POP")
		emit_signal("set_destination_mode","POP")
	if is_active and event is InputEventKey and event.is_action_released("queue_destinations"):
		var socket = ServerNetwork.get(client_id)
		socket.set_destination_mode(id,"FORWARD")
		emit_signal("set_destination_mode","FORWARD")
	if is_active and event is InputEventKey and event.is_action_released("toggle_gravity"):
		var socket = ServerNetwork.get(client_id)
		socket.toggle_gravity(id)
	if is_active and event is InputEventKey and event.is_action_released("toggle_destinations"):
		var socket = ServerNetwork.get(client_id)
		socket.toggle_destinations(id)
	if is_active and event is InputEventKey and event.is_action_released("control"):
		var socket = ServerNetwork.get(client_id)
		socket.clear_destinations(id)
	if is_active and event is InputEventKey and event.is_action_released("smack"):
		socket.ability(client_id,0)	
		GlobalSignalsClient.activate_ability(client_id,0)

	if is_active and event is InputEventKey and event.is_action_released("globular_teleport_base"):
		AbilityAPI.globular_teleport().add_base(client_id,body.global_transform.origin)
	if is_active and event is InputEventKey and event.is_action_released("globular_teleport_point"):
		AbilityAPI.globular_teleport().add_point(client_id,body.global_transform.origin)
	if is_active and event is InputEventKey and event.is_action_released("globular_teleport_send"):
		AbilityAPI.globular_teleport().do(client_id)
		GlobalSignalsClient.activate_ability(client_id,1)

	if is_active and event is InputEventKey:
		var vec = get_input_vec()
		pointer.position(body.global_transform.origin - vec)
		physics_socket.send_input(id,vec)
	
var position_proc = 0
var position_mod = 60
func _process(delta):
	if is_active:
		camera_root.global_transform.origin = body.global_transform.origin
		var vec = get_input_vec()
		pointer.position(body.global_transform.origin - vec)
		muh_process()
		
	if position_proc%position_mod == 0:
		position_proc = 0	
		GlobalSignalsClient.player_position(body.global_transform.origin)
	position_proc += 1

func muh_process():
	if is_active:
		var vec = get_input_vec()
		physics_socket.send_input(id,get_input_vec())
		
func get_input_vec() -> Vector3:
	var diff = camera.global_transform.origin - self.body.global_transform.origin
	#represenets a vector pointing away from our body horizontally, in the direction the camera is facing
	var pointer:Vector3 = Vector3(diff.x,0,diff.z).normalized()
	var vec = Vector3(0, 0 , 0)
	if Input.is_action_pressed("forward",true):
		vec -= pointer
	if Input.is_action_pressed("left",true):
		vec += pointer.rotated(Vector3.UP,3*PI/2)
	if Input.is_action_pressed("right",true):
		vec -= pointer.rotated(Vector3.UP,3*PI/2)
	if Input.is_action_pressed("backward",true):
		vec += pointer
	if Input.is_action_pressed("rise",true):
		vec += Vector3.UP
	if Input.is_action_pressed("fall",true):
		vec += Vector3.DOWN
		
	return vec#.normalized()

func set_active(active:bool):
	print_debug("player active:",id," ",active)
	self.is_active = active
	camera.set_active(active)
	
func _handle_message(msg,delta_accum):
	self.default_handle_message(msg,delta_accum)

func _physics_process(delta):
	self.default_physics_process(delta)
