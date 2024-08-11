extends ClientPlayerEntity

class_name Player
onready var camera_root =find_node("CameraRoot")
onready var camera:Camera = camera_root.find_node("Camera")
onready var curserRay:CursorRay = camera_root.find_node("CursorRay")
onready var pointer:PlayerPathPointer = PlayerPathPointer.new()
onready var terrain_scanner:Timer = Timer.new()
onready var input_timer:Timer = Timer.new()
var is_active = false

func _ready():
	self.is_npc = false
	body.add_child(pointer)
	physics_socket = ServerNetwork.get_physics(client_id)
	assert(physics_socket!=null)
	terrain_scanner.wait_time = 6
	terrain_scanner.connect("timeout",self,"scan_for_terrain")
	self.add_child(terrain_scanner)
	input_timer.wait_time = 0.1
	input_timer.connect("timeout",self,"muh_process")
	self.add_child(input_timer)
	input_timer.start()
	#terrain_scanner.start()
	#curserRay.connect("intersection_clicked",self,"handle_clicked")

func scan_for_terrain():
	if is_active:
		print_debug("Scanning for terrain for player " , client_id)
		ServerNetwork.get(client_id).get_top_level_terrain_in_distance(1024,self.global_transform.origin)
		
func _input(event):
	if is_active and event is InputEventKey and event.is_action_released("toggle_gravity"):
		var socket = ServerNetwork.get(client_id)
		socket.toggle_gravity(id)
	if is_active and event is InputEventKey and event.is_action_released("toggle_destinations"):
		var socket = ServerNetwork.get(client_id)
		socket.toggle_destinations(id)
	if is_active and event is InputEventKey and event.is_action_released("control"):
		var socket = ServerNetwork.get(client_id)
		socket.clear_destinations(id)
	if is_active and event is InputEventKey:
		var vec = get_input_vec2()#get_input_vec(event)
		pointer.position(body.global_transform.origin - vec)
		#physics_socket.send_input(id,vec)
		#print("input ", vec)

func _process(delta):
	if is_active:
		camera_root.global_transform.origin = body.global_transform.origin
		var vec = get_input_vec2()
		pointer.position(body.global_transform.origin - vec)
		
func muh_process():
	if is_active:
		#to not have to set this every frame we have to have timeout on physics server
		var vec = get_input_vec2()
		pointer.position(body.global_transform.origin - vec)
		physics_socket.send_input(id,get_input_vec2())
		
func get_input_vec(event) -> Vector3:
	var diff = camera.global_transform.origin - self.body.global_transform.origin
	#represenets a vector pointing away from our body horizontally, in the direction the camera is facing
	var pointer:Vector3 = Vector3(diff.x,0,diff.z).normalized()
	var vec = Vector3(0, 0 , 0)
	if event is InputEventKey and event.is_action_pressed("forward",true):
		vec -= pointer
	if event is InputEventKey and event.is_action_pressed("left",true):
		vec += pointer.rotated(Vector3.UP,3*PI/2)
	if event is InputEventKey and event.is_action_pressed("right",true):
		vec -= pointer.rotated(Vector3.UP,3*PI/2)
	if event is InputEventKey and event.is_action_pressed("backward",true):
		vec += pointer
	if event is InputEventKey and event.is_action_pressed("rise",true):
		vec += Vector3.UP
	if event is InputEventKey and event.is_action_pressed("fall",true):
		vec += Vector3.DOWN
		
	return vec#.normalized()

func get_input_vec2() -> Vector3:
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
	
#can probably remove this entirely now once submenus are in
func handle_clicked(position,button_index):
	if button_index == 1 and curserRay.intersect_position != null:
		var y = curserRay.intersect_position.y
		var x = curserRay.intersect_position.x
		var z = curserRay.intersect_position.z
		var dest = Vector3(x,y,z)
		if id != null:
			print_debug("setting destination for player:",dest)
			#ServerNetwork.get(id).add_destination(id,dest,"WAYPOINT")
