extends Node


# Declare member variables here. Examples:
# var a = 2
# var b = "text"

onready var camera_root = self.find_parent("CameraRoot")
onready var camera_body = self.find_parent("CameraBody")
onready var this = camera_body.find_node("Camera")
onready var raycast = self.find_node("CursorRay")
var minx = 0
var max_x = 90
var delta_x = 0
var sensitivity_x = 1
var sensitivity_y = 25
var rotating = false
func _input(event):
	if event is InputEventMouseMotion and rotating:
		var rot = Vector2(event.relative.y * sensitivity_y,event.relative.x * sensitivity_x) * delta_x
		camera_root.rotation_degrees.x -= rot.x
		#camera_root.rotation_degrees.x = clamp(camera_root.rotation_degrees.x,minx,max_x)
		camera_root.global_rotate(Vector3.UP,-rot.y)
		camera_body.look_at(camera_root.global_transform.origin,Vector3.UP)
	if event is InputEventMouseButton and event.is_action("scroll_in"):
		var backdir = (camera_root.global_transform.origin - camera_body.global_transform.origin).normalized() * 1 #* scrollDelta
		camera_body.global_transform.origin += backdir
		pass
	if event is InputEventMouseButton and event.is_action("scroll_out"):
		var backdir = (camera_root.global_transform.origin - camera_body.global_transform.origin).normalized() * -1 #* scrollDelta
		camera_body.global_transform.origin += backdir
		pass
	if event is  InputEventMouseButton and event.is_action_pressed("rotate_camera"):
		rotating = true
	if event is  InputEventMouseButton and event.is_action_released("rotate_camera"):
		rotating = false
	if event is InputEventMouseButton and event.pressed and event.button_index == 1:
		 
		print(raycast.intersect_position,raycast.intersect_object)
	
func _physics_process(delta):
	delta_x = delta
	#print("trasnform:" + str(camera_root.global_transform))
	pass
# Called when the node enters the scene tree for the first time.
func _ready():
	OS.window_fullscreen = true
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
#func _process(delta):
#	pass
