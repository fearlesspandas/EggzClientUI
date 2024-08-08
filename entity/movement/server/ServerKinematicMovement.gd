extends Movement

class_name ServerKinematicMovement

onready var entity:ServerEntity = get_parent()
var speed = 0.0
var body_ref

func teleport(location:Vector3,body:KinematicBody):
	body.translate(location - body.global_transform.origin)
	
	
func move(delta,location:Vector3,body:KinematicBody):
	var diff_base:Vector3 = (body.global_transform.origin - location)
	var diff:Vector3 = (body.global_transform.origin - location).normalized() * speed * delta
	var diff2:Vector3 = diff_base * diff_base.length() * speed * delta
	body.move_and_collide(-diff* 0.0005)
		
func move_by_gravity(delta,location:Vector3,body:KinematicBody):
	var diff_base:Vector3 = (body.global_transform.origin - location)
	var diff:Vector3 = (body.global_transform.origin - location).normalized() * speed * delta
	var diff2:Vector3 = diff_base * diff_base.length() * speed * delta
	body.move_and_collide(-diff2* 0.00000001)
	
func apply_vector(delta,vector:Vector3,body:KinematicBody):
	#body.global_transform.origin += vector.normalized() * speed * delta * 0.1
	var v = vector.normalized() * speed * delta * 0.001
	#must be called every frame for collision to work
	#if body.move_and_collide(v,false) == null:
	#		pass
	body.move_and_slide(v * 100,Vector3.UP)
	
			#body.move_and_collide(v)
	#body.move_and_collide()

func set_max_speed(max_speed):
	if max_speed != null:
		speed = max_speed
		
func _ready():
	pass
