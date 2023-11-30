extends ClientPlayerEntity


onready var camera_root =find_node("CameraRoot")
onready var camera:Camera = camera_root.find_node("Camera")
onready var curserRay:RayCast = camera_root.find_node("CursorRay")
var is_active = false
func _input(event):
	if is_active and event is InputEventMouseButton and event.is_action_pressed("left_click") and curserRay.intersect_position != null:
		#print("attempting to add destination")
		var y = curserRay.intersect_position.y
		var x = curserRay.intersect_position.x
		var z = curserRay.intersect_position.z
		var dest = Vector3(x,y,z)
		if id != null:
			print("setting destination for player:",dest)
			ServerNetwork.get(id).add_destination(id,dest)
	elif is_active and event is InputEventKey:
		var vec = get_input_vec(event)
		var socket = ServerNetwork.get(id)
		if socket != null:
			socket.send_input(id,vec)
			
func get_input_vec(event) -> Vector3:
	var diff = camera.global_transform.origin - self.body.global_transform.origin
	#represenets a vector pointing away from our body horizontally, in the direction the camera is facing
	var pointer:Vector3 = Vector3(diff.x,body.global_transform.origin.y,diff.z).normalized()
	var vec = Vector3(0, 0 , 0)
	if event is InputEventKey and event.is_action_pressed("forward"):
		vec -= pointer
	if event is InputEventKey and event.is_action_pressed("left"):
		vec += pointer.rotated(Vector3.UP,3*PI/2)
	if event is InputEventKey and event.is_action_pressed("right"):
		vec -= pointer.rotated(Vector3.UP,3*PI/2)
	if event is InputEventKey and event.is_action_pressed("backward"):
		vec += pointer
	if event is InputEventKey and event.is_action_pressed("rise"):
		vec += Vector3.UP
	if event is InputEventKey and event.is_action_pressed("fall"):
		vec += Vector3.DOWN
	return vec#.normalized()
	
func _physics_process(delta):
	pass
	
func set_active(active:bool):
	print("player active:",id,active)
	is_active = active
	camera.set_active(active)
	
func _ready():
	pass # Replace with function body.
