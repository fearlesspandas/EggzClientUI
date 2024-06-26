extends ClientPlayerEntity

class_name Player
onready var camera_root =find_node("CameraRoot")
onready var camera:Camera = camera_root.find_node("Camera")
onready var curserRay:CursorRay = camera_root.find_node("CursorRay")
onready var pointer:PlayerPathPointer = PlayerPathPointer.new()
var is_active = false

func _ready():
	
	self.add_child(pointer)
	curserRay.connect("intersection_clicked",self,"handle_clicked")
func _input(event):
	if is_active and event is InputEventMouseButton and event.is_action_pressed("left_click") and curserRay.intersect_position != null:
		#print("attempting to add destination")
		#var y = curserRay.intersect_position.y
		#var x = curserRay.intersect_position.x
		#var z = curserRay.intersect_position.z
		#var dest = Vector3(x,y,z)
		#if id != null:
	#		print("setting destination for player:",dest)
	#		ServerNetwork.get(id).add_destination(id,dest,"WAYPOINT")
		pass
	if is_active and event is InputEventKey and event.is_action_released("control"):
		var socket = ServerNetwork.get(client_id)
		if socket != null:
			#id and client id should be the same but
			#this code is technically more general
			socket.clear_destinations(id)
	if is_active and event is InputEventKey:
		var vec = get_input_vec(event)
		var socket = ServerNetwork.get(client_id)
		pointer.position(body.global_transform.origin - vec)
		if socket != null:
			socket.send_input(id,vec)
			
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

func set_active(active:bool):
	print("player active:",id,active)
	is_active = active
	camera.set_active(active)
	
#can probably remove this entirely now once submenus are in
func handle_clicked(position,button_index):
	if button_index == 1 and curserRay.intersect_position != null:
		var y = curserRay.intersect_position.y
		var x = curserRay.intersect_position.x
		var z = curserRay.intersect_position.z
		var dest = Vector3(x,y,z)
		if id != null:
			print("setting destination for player:",dest)
			#ServerNetwork.get(id).add_destination(id,dest,"WAYPOINT")
