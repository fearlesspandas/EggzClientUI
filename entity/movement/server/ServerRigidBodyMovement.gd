extends Movement

class_name ServerRigidMovement


var speed = 0.5
var speed_limit = 10
var autopilot_speed_limit = 0.0005
var stopped_location = null

var x_velocity = 0
var y_velocity = 0
var z_velocity = 0
var in_motion

func move(delta,location:Vector3,body:RigidBody):
	var base = (body.global_transform.origin - location)
	var diffvec:Vector3 = (body.global_transform.origin - location).normalized() * speed * delta * base.length()/3
	diffvec = diffvec * clamp(diffvec.length(),0,min(autopilot_speed_limit,speed_limit)/(1/(speed_limit + 1)))/diffvec.length()
	body.set_axis_velocity(Vector3(1,0,0) * -diffvec.x)
	body.set_axis_velocity(Vector3(0,1,0) * -diffvec.y)
	body.set_axis_velocity(Vector3(0,0,1) * -diffvec.z)
	
func stop(body:RigidBody):
	Input
	#body.add_central_force(-body.linear_velocity)
	#body.angular_velocity = Vector3.ZERO
	#body.apply_central_impulse(-body.linear_velocity)
	#body.add_central_force(-body.linear_velocity)
	#print("stopping")
	body.sleeping = true

func decelerate(value:float,decell:float) -> float:
	if value > 0:
		value -= min(decell,value) 
	if value < 0:
		value += min(decell,-value)
	return value
	
func handle_input_vec(vector:Vector3,accell:float,decell:float):
	if vector == Vector3.ZERO:
		in_motion = false
		x_velocity = decelerate(x_velocity,decell)
		z_velocity = decelerate(z_velocity,decell)
	else:
		in_motion = true
		x_velocity -= vector.x * accell
		z_velocity -= vector.z * accell

func move_along_path(vector:Vector3,body:RigidBody):
	var normal = vector.normalized()	
	var should_jump = vector.y != 0
	var vec = Vector3(normal.x * x_velocity,int(should_jump) * -0.01 * vector.y,normal.z * z_velocity)
	if vec.length() > 0 and speed_limit != null:
		vec = vec * clamp(vec.length(),0,speed_limit)/vec.length() 
	body.add_central_force(-vec)
	#body.set_axis_velocity(-vec)
	

func apply_vector(delta,vector:Vector3,body:RigidBody):
	handle_input_vec(vector,0.005,20)
	#if vector != Vector3.ZERO:
	move_along_path(Vector3(1,vector.y,1),body)
	
func apply_vector2(delta,vector:Vector3,body:RigidBody):
	var dir:Vector3 = vector*delta * speed
	var curr = body.linear_velocity
	#if curr.length() < speed_limit:
	dir = dir * clamp(dir.length(),0,speed_limit)/dir.length() 
	#body.global_transform.origin += dir
	#body.add_central_force(dir)
	if dir.y > 0:
		body.apply_central_impulse(Vector3.UP * 0.01)
		dir.y = 0
	body.set_axis_velocity(dir)

func get_lv(body:RigidBody) -> Vector3:
	return body.linear_velocity

func set_max_speed(max_speed):
	speed_limit = max_speed
	
func _process(delta):
	if !in_motion:
		x_velocity = decelerate(x_velocity,0.1)
		z_velocity = decelerate(z_velocity,0.1)

func _ready():
	pass
