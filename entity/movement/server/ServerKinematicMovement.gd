extends Movement

class_name ServerKinematicMovement

onready var entity:ServerEntity = get_parent()

var speed = 0.0
var body_ref
var dir = Vector3()


func teleport(location:Vector3,body:KinematicBody):
	body.translate(location - body.global_transform.origin)
	
func move(delta,location:Vector3,body:KinematicBody):
	var diff:Vector3 = (body.global_transform.origin - location).normalized() * speed * delta
	dir = -diff* 0.0005 * 10000
	body.move_and_slide(dir,Vector3.UP)
		
func move_by_gravity2(delta,location:Vector3,body:KinematicBody):
	var diff:Vector3 = (body.global_transform.origin - location).normalized() * speed * delta
	dir += -diff* 0.0005 * 100
	body.move_and_slide(dir,Vector3.UP)

func move_by_gravity(id,delta,location:Vector3,body:KinematicBody):
	var diff:Vector3 = (body.global_transform.origin - location).normalized() * speed * delta
	physics_socket.send_input(id,-diff * 0.05 * 10)
	
func apply_vector(delta,vector:Vector3,body:KinematicBody):
	#body.global_transform.origin += vector.normalized() * speed * delta * 0.1
	var v = vector.normalized() * speed * delta * 0.001
	#must be called every frame for collision to work
	#if body.move_and_collide(v,false) == null:
	#		pass
	dir += v * 100
	body.move_and_slide(dir,Vector3.UP)

func move_by_direction(delta,body:KinematicBody):
	body.move_and_slide(dir * speed * 0.005,Vector3.UP)

func get_direction() -> Vector3:
	return dir
	
func apply_direction(direction:Vector3):
	dir += direction.normalized()

func set_direction(direction:Vector3):
	dir = direction
	
func set_max_speed(max_speed):
	if max_speed != null:
		speed = max_speed
		
func get_max_speed() -> float:
	return speed
func _ready():
	pass
