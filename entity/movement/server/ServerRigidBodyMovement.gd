extends Movement

class_name ServerRigidMovement


var speed = 0.5
var speed_limit = 10
var autopilot_speed_limit = 0.05
func move(delta,location:Vector3,body:RigidBody):
	var base = (body.global_transform.origin - location)
	var diffvec:Vector3 = (body.global_transform.origin - location).normalized() * speed * delta * base.length()/3
	diffvec = diffvec * clamp(diffvec.length(),0,autopilot_speed_limit)/diffvec.length()
	body.set_axis_velocity(Vector3(1,0,0) * -diffvec.x)
	body.set_axis_velocity(Vector3(0,1,0) * -diffvec.y)
	body.set_axis_velocity(Vector3(0,0,1) * -diffvec.z)
	
func stop(delta,body:RigidBody):
	#body.sleeping = true
	body.add_central_force(-body.linear_velocity)
	
func apply_vector(delta,vector:Vector3,body:RigidBody):
	var dir:Vector3 = vector*delta * speed
	var curr = body.linear_velocity
	#if curr.length() < speed_limit:
	dir = dir * clamp(dir.length(),0,speed_limit)/dir.length() 
	#body.add_central_force(dir)
	body.set_axis_velocity(dir)

func get_lv(body:RigidBody) -> Vector3:
	return body.linear_velocity

func set_max_speed(max_speed):
	speed_limit = max_speed
	
func _ready():
	pass
