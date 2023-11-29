extends Movement

class_name ServerKinematicMovement


var speed = 0.1

func move(delta,location:Vector3,body:KinematicBody):
	var diff_base:Vector3 = (body.global_transform.origin - location)
	var diff:Vector3 = (body.global_transform.origin - location).normalized() * speed * delta
	var diff2:Vector3 = diff_base * diff_base.length() * speed * delta
	body.global_transform.origin -= diff2
	#body.move_and_collide(-diff)
func _ready():
	pass
