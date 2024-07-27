extends Movement

class_name ServerRigidMovement


var speed = 0.5
var speed_limit = 10
var autopilot_speed_limit = 0.0005
var stopped_location = null
var G = 0.1
var x_velocity = 0
var y_velocity = 0
var z_velocity = 0
var in_motion

var force_vec_accum : Vector3 = Vector3.ZERO
var body_ref:RigidBody


func linear_move(delta,location:Vector3,body:RigidBody):
	body.mode = RigidBody.MODE_KINEMATIC
	var base = location - body.global_transform.origin
	var dir = base.normalized()
	if speed_limit == null:
		speed_limit = 0
	body.global_transform.origin += dir*speed_limit*delta * 0.0005
	
func gravity_move(delta,location:Vector3,body:RigidBody):
	var base = location - body.global_transform.origin
	var dir = base.normalized()
	if speed_limit == null:
		speed_limit = 0
	body.set_linear_velocity((dir*speed_limit*G/base.length_squared()))

func move(delta,location:Vector3,body:RigidBody):
	var base = (body.global_transform.origin - location)
	var diffvec:Vector3 = (body.global_transform.origin - location).normalized() * speed * delta * base.length()/3
	if speed_limit == null:
		speed_limit = 0
	assert(speed_limit != null)
	
	diffvec = diffvec * clamp(diffvec.length(),0,min(autopilot_speed_limit,speed_limit)/(1/(speed_limit + 1)))/diffvec.length()
	#diffvec = diffvec * 10000 * clamp(diffvec.length(),0,min(autopilot_speed_limit,speed_limit)/(1/(speed_limit + 1)))/base.length_squared()
	#body.set_axis_velocity(Vector3(1,0,0) * -diffvec.x)
	#body.set_axis_velocity(Vector3(0,1,0) * -diffvec.y)
	#body.set_axis_velocity(Vector3(0,0,1) * -diffvec.z)
	#body.apply_central_impulse(-diffvec)
	body.set_linear_velocity(body.linear_velocity -diffvec * 0.1)

	#apply_vector(delta,-base * min(autopilot_speed_limit,speed_limit)/(1/(speed_limit + 1)),body)
	
func stop(body:RigidBody):
	in_motion = false
	#body.add_central_force(-force_vec_accum)
	force_vec_accum = Vector3.ZERO
	if body.linear_velocity.length() > 0:
		pass #print_debug("Linear velocity", body.linear_velocity)
		#body.set_linear_velocity(Vector3.ZERO)
		#body.apply_central_impulse(-body.linear_velocity)
	#body.sleeping = true

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
	var vec = Vector3(normal.x * x_velocity,int(should_jump) * -0.05 * vector.y,normal.z * z_velocity)
	if vec.length() > 0 and speed_limit != null:
		vec = vec * clamp(vec.length(),0,speed_limit)/vec.length() 
	#body.add_central_force(-vec)
	force_vec_accum -= vec #* 0.01
	#going back and forth between add_force vs apply_impulse. forces feel more chaotic sometimes, but are frame independent
	#body.apply_central_impulse(-vec * 0.01)
	body.set_linear_velocity(body.linear_velocity + (-vec * 0.01))
	

func apply_vector(delta,vector:Vector3,body:RigidBody):
	handle_input_vec(vector,0.010,20)
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
	
func _physics_process(delta):
	if !in_motion:
		x_velocity = decelerate(x_velocity,0.1)
		z_velocity = decelerate(z_velocity,0.1)
	#move_along_path(Vector3(1,0,1),body_ref)

func _ready():
	pass
