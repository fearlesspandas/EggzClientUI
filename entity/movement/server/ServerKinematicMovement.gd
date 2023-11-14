extends Node

class_name ServerKinematicMovement

# Declare member variables here. Examples:
# var a = 2
# var b = "text"
var speed = 0.001
func move(delta,location:Vector3,body:KinematicBody):
	var diff:Vector3 = (body.global_transform.origin - location).normalized() * speed * delta
	body.move_and_collide(diff,false)
# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
#func _process(delta):
#	pass
