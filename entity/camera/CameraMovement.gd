extends Node


# Declare member variables here. Examples:
# var a = 2
# var b = "text"

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
var minzoom = 30
var is_active:bool = false
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
			#camera_root.rotation_degrees.x = clamp(camera_root.rotation_degrees.x,minx,max_x)
			camera_root.global_rotate(Vector3.UP,-rot.y)
			
		if event is InputEventMouseButton and event.is_action("scroll_in"):
			var diff = camera_root.global_transform.origin - camera_body.global_transform.origin
			if diff.length() > maxzoom:
				var backdir = diff.normalized() * 1 #* scrollDelta
				camera_body.global_transform.origin += backdir
			pass
		if event is InputEventMouseButton and event.is_action("scroll_out"):
			var diff = camera_root.global_transform.origin - camera_body.global_transform.origin
			if diff.length() < minzoom: 
				var backdir = diff.normalized() * -1 #* scrollDelta
				camera_body.global_transform.origin += backdir
			pass
		if event is  InputEventMouseButton and event.is_action_pressed("rotate_camera"):
			rotating = true
		if event is  InputEventMouseButton and event.is_action_released("rotate_camera"):
			rotating = false
		if event is InputEventMouseButton and event.pressed and event.button_index == 1:
			 
			print("collisionDetected:",raycast.intersect_position,raycast.intersect_object)
	
func _physics_process(delta):
	delta_x = delta
	camera_body.look_at(camera_root.global_transform.origin,Vector3.UP)
	#print("trasnform:" + str(camera_root.global_transform))
	pass
# Called when the node enters the scene tree for the first time.
func _ready():
	OS.window_fullscreen = false
	pass # Replace with function body.
func set_active(active:bool):
	is_active = active

# Called every frame. 'delta' is the elapsed time since the previous frame.
#func _process(delta):
#	pass
