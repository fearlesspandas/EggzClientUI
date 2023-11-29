extends Movement

class_name ServerRigidMovement


var speed = 0.5
var initialized = false
func move(delta,location:Vector3,body:RigidBody):
	var base = (body.global_transform.origin - location)
	var diffvec:Vector3 = (body.global_transform.origin - location).normalized() * speed * delta * base.length()/3
	
	body.set_axis_velocity(Vector3(1,0,0) * -diffvec.x)
	body.set_axis_velocity(Vector3(0,1,0) * -diffvec.y)
	body.set_axis_velocity(Vector3(0,0,1) * -diffvec.z)
	
func stop(delta,body:RigidBody):
	body.set_axis_velocity(Vector3(0,0,0))
	
func apply_vector(delta,vector:Vector3,body:RigidBody):
	body.set_axis_velocity(vector*delta)
	
func _ready():
	pass
