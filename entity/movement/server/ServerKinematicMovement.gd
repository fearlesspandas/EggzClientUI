extends Movement

class_name ServerKinematicMovement


var speed = 0.0
var body_ref
func move(delta,location:Vector3,body:KinematicBody):
	var diff_base:Vector3 = (body.global_transform.origin - location)
	var diff:Vector3 = (body.global_transform.origin - location).normalized() * speed * delta
	var diff2:Vector3 = diff_base * diff_base.length() * speed * delta
	#body.global_transform.origin -= diff2 * 0.0000001
	body.move_and_collide(-diff2* 0.00000001)
	
func apply_vector(delta,vector:Vector3,body:KinematicBody):
	#body.global_transform.origin += vector.normalized() * speed * delta * 0.1
	var v = vector.normalized() * speed * delta * 0.001
	#must be called every frame for collision to work
	if body.move_and_collide(v,true) == null:
			pass
			#body.move_and_collide(v)
				
func set_max_speed(max_speed):
	if max_speed != null:
		speed = max_speed
func _ready():
	pass
