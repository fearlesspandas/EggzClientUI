extends Node


func find_mouse_collision(camera:Camera,world:World,position:Vector2) -> Vector3:
	var from = camera.project_ray_origin(position)
	var to = from + camera.project_ray_normal(position) * 1000
	var mouse_position = get_viewport().get_mouse_position()
	var ray_origin = from
	var ray_target = to
	var space_state = world.direct_space_state
	var intersection = space_state.intersect_ray(ray_origin,ray_target)
	return intersection.position
	
