extends Movement

class_name ClientKinematicMovement

var speed = 200
func move(delta,location:Vector3,body:KinematicBody):
	#print("moving:",location)
	var diff:Vector3 = (body.global_transform.origin - location).normalized() * speed * delta
	#body.move_and_collide(-diff,false)
	#body.move_and_slide_with_snap(-diff,Vector3.UP)
	body.global_transform.origin = location
# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
#func _process(delta):
#	pass
