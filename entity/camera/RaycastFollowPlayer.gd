extends RayCast


class_name CursorRay
signal intersection_clicked(intersect_position,button)
signal object_clicked(object,button)
signal body_entered(object)
onready var camera_root = find_parent("CameraRoot")
onready var camera = find_parent("Camera")
onready var this = camera.find_node("CursorRay")
var from = null
var to = null
var intersect_position = null
var intersect_object = null
var hovered_position = null
var hovered_object = null
func _input(event):
	if event is InputEventMouseButton and !(camera.check_mouse_position_within_viewport(event.position) and camera.is_active):
		intersect_object = null
		intersect_position = null
	elif event is InputEventMouseButton and event.pressed and (event.button_index == 1 || event.button_index == 2):
		print_debug("Looking for intersection")
		from = camera.project_ray_origin(event.position)
		to = from + camera.project_ray_normal(event.position) * camera.far
		var mouse_position = get_viewport().get_mouse_position()
		var ray_origin = from
		var ray_target = to
		var space_state = get_world().direct_space_state
		var intersection = space_state.intersect_ray(ray_origin,ray_target)
		if not intersection.empty() and intersection.position != intersect_position:
			intersect_position = intersection.position
			intersect_object = intersection.collider
			DataCache.add_data('camera','intersect_position',intersection.position)
			DataCache.add_data('camera','intersect_object',intersection.collider)
			print_debug("Intersection clicked " , intersect_position, intersect_object)
			emit_signal("intersection_clicked",intersect_position,event.button_index)
			if intersect_object.has_method("clicked"):
				intersect_object.clicked(event.position,intersect_position)
		else:
			DataCache.remove_data('camera','intersect_position')
			DataCache.remove_data('camera','intersect_object')
			intersect_object= null
			intersect_position = null
		from = null
		to = null

	elif event is InputEventMouseMotion:
		from = camera.project_ray_origin(event.position)
		to = from + camera.project_ray_normal(event.position) * camera.far
		var mouse_position = get_viewport().get_mouse_position()
		var ray_origin = from
		var ray_target = to
		var space_state = get_world().direct_space_state
		var intersection = space_state.intersect_ray(ray_origin,ray_target)
		if not intersection.empty():
			if hovered_object != null and hovered_object != intersection.collider and hovered_object.has_method("exited"):
				hovered_object.exited()
			if hovered_object == null || hovered_object != intersection.collider:
				hovered_position = intersection.position
				hovered_object = intersection.collider
				emit_signal("body_entered",hovered_position)
				if hovered_object.has_method("entered"):
					hovered_object.entered()
		else:
			if hovered_object != null and hovered_object.has_method("exited"):
				hovered_object.exited()
			hovered_object = null
			hovered_position = null
		from = null
		to = null

	
func _physics_process(delta):
	pass
func _ready():
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
#func _process(delta):
#	pass
