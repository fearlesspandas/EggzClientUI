extends Camera


onready var camera_root = self.find_parent("CameraRoot")
onready var camera_body = self.find_parent("CameraBody")
onready var this = camera_body.find_node("Camera")
onready var raycast = self.find_node("CursorRay")
var minx = -90
var max_x = 90
var delta_x = 0
var sensitivity_x = 0.5
var sensitivity_y = 25
var rotating = false
var maxzoom = 5
var minzoom = 120
var is_active:bool = false
var is_following:bool = false

func check_mouse_position_within_viewport(position:Vector2) -> bool:
	var viewport:Viewport = get_viewport()
	var mins = viewport.global_canvas_transform.origin
	var maxs = viewport.global_canvas_transform.origin + viewport.size
	return mins.x < position.x and mins.y < position.y and maxs.x > position.x and maxs.y > position.y

func _input(event):
	if event is InputEventMouse and check_mouse_position_within_viewport(event.position) and is_active:
		if event is InputEventMouseMotion and rotating:
			var rot = Vector2(event.relative.y * sensitivity_y,event.relative.x * sensitivity_x) * delta_x
			camera_root.rotation_degrees.x -= rot.x
			camera_root.global_rotate(Vector3.UP,-rot.y)
		if event is InputEventMouseButton and event.is_action("scroll_in"):
			var diff = camera_root.global_transform.origin - camera_body.global_transform.origin
			if diff.length() > maxzoom:
				var backdir = diff.normalized() * 1 
				camera_body.global_transform.origin += backdir
			pass
		if event is InputEventMouseButton and event.is_action("scroll_out"):
			var diff = camera_root.global_transform.origin - camera_body.global_transform.origin
			if diff.length() < minzoom: 
				var backdir = diff.normalized() * -1 
				camera_body.global_transform.origin += backdir
			pass
		if event is  InputEventMouseButton and event.is_action_pressed("rotate_camera"):
			rotating = true
		if event is  InputEventMouseButton and event.is_action_released("rotate_camera"):
			rotating = false
		if event is InputEventMouseButton and event.pressed and event.button_index == 1:
			print_debug("collisionDetected:",raycast.intersect_position,raycast.intersect_object)
	if event is InputEventKey and event.is_action_released("camera_look_towards"):
		is_following = !is_following
	
func _physics_process(delta):
	delta_x = delta
	if is_active:
		camera_body.look_at(camera_root.global_transform.origin,Vector3.UP)
	if is_active and is_following:
		follow_dest()	

func follow_dest():
	var dest = ClientReferences.destination_manager.get_active_destination()
	if dest != null:
		camera_root.look_at(dest.location,Vector3.UP)
func _ready():
	OS.window_fullscreen = false
	pass # Replace with function body.

func set_active(active:bool):
	is_active = active
		
#deprecated
func look_towards(location:Vector3):
	var diff = location - camera_root.global_transform.origin
	var orig_location = camera_body.global_transform.origin
	var orig_distance = (camera_root.global_transform.origin - camera_body.global_transform.origin).length()
	camera_body.look_at(location,Vector3.UP)
	camera_body.global_transform.origin = camera_root.global_transform.origin - (diff.normalized() * orig_distance)
	#if orig_location.y >= camera_body.global_transform.origin.y:
	#	camera_body.global_transform.origin.y = orig_location.y
	

