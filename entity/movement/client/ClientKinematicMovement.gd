extends Movement

class_name ClientKinematicMovement

var speed = 2
var dir:Vector3
func move(delta,location:Vector3,body:KinematicBody):
	#print("moving:",location)
	var diff = body.global_transform.origin - location
	var diff_normalized:Vector3 = diff.normalized() * speed * delta
	#body.move_and_collide(-diff,false)
	#body.move_and_slide_with_snap(-diff,Vector3.UP)
	body.global_transform.origin = location

func set_direction(direction:Vector3):
	dir = direction

func move_by_direction(delta,body:KinematicBody):
	#body.move_and_slide(dir* delta * 0.05,Vector3.UP)
	body.move_and_slide(-dir*0.0005 * 20000,Vector3.UP)

func _ready():
	pass # Replace with function body.
