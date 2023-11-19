extends RayCast



onready var camera_root = find_parent("CameraRoot")
onready var camera = find_parent("Camera")
onready var this = camera.find_node("CursorRay")
var from = null
var to = null
var intersect_position = null
var intersect_object = null
func _input(event):
	if event is InputEventMouseButton and !(camera.check_mouse_position_within_viewport(event.position) and camera.is_active):
		intersect_object = null
		intersect_position = null
	elif event is InputEventMouseButton and event.pressed and event.button_index == 1:
		from = camera.project_ray_origin(event.position)
		to = from + camera.project_ray_normal(event.position) * 1000
		var mouse_position = get_viewport().get_mouse_position()
		var ray_origin = from
		var ray_target = to
		var space_state = get_world().direct_space_state
		var intersection = space_state.intersect_ray(ray_origin,ray_target)
		if not intersection.empty():
			intersect_position = intersection.position
			intersect_object = intersection.collider
		else:
			intersect_object= null
			intersect_position = null
	
func _physics_process(delta):
	pass
func _ready():
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
#func _process(delta):
#	pass
