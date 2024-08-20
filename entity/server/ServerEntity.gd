extends PhysicalPlayerEntity
class_name ServerEntity

onready var message_controller:MessageController = MessageController.new()
onready var timer:Timer = Timer.new()
onready var health_add_timer: Timer = Timer.new()
onready var spawn

var socket:ClientWebSocket
var physics_socket:RustSocket
var requested_dest = false
var timeout = 10
var last_request = null
var destination:Destination = null
var destinations_active:bool
var gravity_active:bool
var epsilon = 3
var isSubbed:bool = false
var is_npc:bool = false
var queued_teleports = []
var queued_input = Vector3()
var is_teleporting:bool = false
var last_pos:Vector3 #used for lv
var lv:Vector3

func _ready():
	spawn = body.global_transform.origin
	self.add_child(message_controller)
	self.movement.body_ref = body
	if is_npc:
		destinations_active = true
		timer.connect("timeout",self,"check_destinations")
		timer.wait_time = 0.5
		self.add_child(timer)
		timer.start()
	else: 
		timer.connect("timeout",self,"timer_polling")
		timer.connect("timeout",self,"check_destinations")
		timer.wait_time = 0.25
		self.add_child(timer)
		timer.start()
		
		#health_add_timer.connect("timeout",self,"add_health")
		#health_add_timer.wait_time = 2
		#self.add_child(health_add_timer)
		#health_add_timer.start()		
	init_sockets()
	
func init_sockets():
	socket = ServerNetwork.get(client_id)
	assert(socket != null)
	physics_socket = ServerNetwork.get_physics(client_id)
	assert(physics_socket != null)
	self.movement.physics_socket = physics_socket

#func add_health():
#	socket.add_health(id,10)
	
func timer_polling():
	socket.set_lv(id,get_lv())
	
func check_destinations():
	socket.get_next_destination(id)
	
func check_dir():
	physics_socket.get_dir_physics(id)

func _handle_message(msg,delta_accum):
	match msg:
		{'GravityActive':{'id': var id, 'is_active':var active}}:
			print_debug("gravity active " , active)
			gravity_active = bool(active)
			if destinations_active and not gravity_active:
				physics_socket.lock_input_physics(id)
				print_debug("input locked " )
			else:
				physics_socket.unlock_input_physics(id)
				print_debug("input unlocked")
		{'DestinationsActive':{'id': var id, 'is_active':var active}}:
			print_debug("destinations active", active)
			destinations_active = bool(active)
			if destinations_active and not gravity_active:
				physics_socket.lock_input_physics(id)
				print_debug("input locked")
			else:
				physics_socket.unlock_input_physics(id)
				print_debug("input unlocked")
		{'NoInput':{'id':var id}}:
			movement.entity_stop(body)
		{'Dir':{'id':var id, 'vec':[var x, var y , var z]}}:
			#print("direction ", Vector3(x,y,z))
			var dir = Vector3(x,y,z)
			var max_speed = DataCache.cached(id,'max_speed')
			if max_speed != null and dir.length() > max_speed:
				dir = dir.normalized() * max_speed
				physics_socket.set_dir_physics(id,dir)
			if (not destinations_active) or gravity_active:
				movement.entity_set_direction(dir)
		{'Input':{"id":var id, "vec":[var x ,var y ,var z]}}:
			queued_input = Vector3(x,y,z)
		{'SET_GLOB_LOCATION':{'id':id,'location':var location}}:
			body.global_transform.origin = location
		{'NextDestination':{'id': var id, 'destination': {'dest_type':var dest_type, 'location':[var x, var y , var z] , 'radius': var radius}}}:
			requested_dest = false
			destination = Destination.new()
			destination.location = Vector3(x,y,z)
			destination.type = dest_type
			destination.radius = radius
		{'TeleportToNext':{'id':var id, 'location':[var x, var y ,var z]}}:
			print_debug("teleporting " , x,y,z)
			queued_teleports.push_front(Vector3(x,y,z))
		{'NoLocation':{'id':var id}}:
			destination = null
		_:
			print("No server entity handler for " , msg)
			pass
	pass
	
func freeze():
	body.global_transform.origin = spawn
	
func update_lv_internal(body,delta):
	if last_pos != null:
		lv = (body.global_transform.origin - last_pos)/delta
	last_pos = body.global_transform.origin
	
func get_lv() -> Vector3:
	if lv != null:
		return lv			
	else:
		return Vector3.ZERO
		
func _physics_process(delta):
	if body is KinematicBody and !is_npc:
		var coll:KinematicCollision = body.get_last_slide_collision()
		if coll != null and coll.collider.has_method("handle_collision"):
			coll.collider.handle_collision(client_id,id)
	physics_socket.get_dir_physics(id)
	update_lv_internal(body,delta)
	#movement.entity_apply_vector(delta,queued_input,body)
	var dir_ = movement.entity_get_direction()
	movement.entity_move_by_direction(delta,body)
	movement.entity_set_max_speed(DataCache.cached(id,'max_speed'))
	if !queued_teleports.empty() and body is KinematicBody:
		var t = queued_teleports.pop_front()
		var dir = (t - body.global_transform.origin)#.normalized()
		body.translate(dir)
	if(destination != null and destinations_active):
		var diff = destination.location - body.global_transform.origin
		match destination.type:
			'{WAYPOINT:{}}':
				if diff.length() > epsilon:
					if gravity_active:
						movement.entity_move_by_gravity(id,delta,destination.location,body)
					else:
						movement.entity_move(delta,destination.location,body)
				else:
					destination = null
			'{TELEPORT:{}}':
				if diff.length() > epsilon:
					if gravity_active:
						movement.entity_move_by_gravity(id,delta,destination.location,body)
					else:
						movement.entity_move(delta,destination.location,body)
				else:
					destination = null
			"{GRAVITY_BIND:{}}":
				if diff.length() > epsilon:
					movement.entity_move_by_gravity(id,delta,destination.location,body)
				else:
					destination = null
					
			_:
				print_debug("no handler found for destination with type ", destination.type)
	else:
		queued_teleports.pop_front()
		is_teleporting = false
	physics_socket.set_location_physics(id,body.global_transform.origin)
	
func _process(delta):
	if !isSubbed:
		var query = PayloadMapper.get_physical_stats(id)
		socket.get_physical_stats(id)
		if !is_npc:
			socket.toggle_destinations(id)
		isSubbed = true


func _input(event):
	if event is InputEventKey and event.is_action_pressed("alt",true):
		movement.entity_stop(body)
	
