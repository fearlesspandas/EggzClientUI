extends Movement

class_name ClientKinematicMovement

var speed = 2
func move(delta,location:Vector3,body:KinematicBody):
	#print("moving:",location)
	var diff = body.global_transform.origin - location
	var diff_normalized:Vector3 = diff.normalized() * speed * delta
	#body.move_and_collide(-diff,false)
	#body.move_and_slide_with_snap(-diff,Vector3.UP)
	if diff.length() < 100:
		body.move_and_slide(-diff,Vector3.UP)
	else:
		body.global_transform.origin = location


func _ready():
	pass # Replace with function body.
