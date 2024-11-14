extends Movement

class_name ClientKinematicMovement

var speed = 2
var dir:Vector3
func move(delta,location:Vector3,body:KinematicBody):
	body.global_transform.origin = location

func move_with_delta(delta,location:Vector3,body:KinematicBody):
	body.global_transform.origin = delta * (location - body.global_transform.origin)

func set_direction(direction:Vector3):
	dir = direction
func get_direction():
	return self.dir
func move_by_direction(delta,body:KinematicBody):
	#body.move_and_slide(dir* delta * 0.05,Vector3.UP)
	body.move_and_slide(-dir*0.0005 * 5000,Vector3.UP)

func _ready():
	pass # Replace with function body.
