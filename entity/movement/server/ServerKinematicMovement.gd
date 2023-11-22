extends Movement

class_name ServerKinematicMovement


var speed = 0.05

func move(delta,location:Vector3,body:KinematicBody):
	var diff:Vector3 = (body.global_transform.origin - location).normalized() * speed * delta
	body.global_transform.origin -= diff
	
func _ready():
	pass
