extends PhysicalPlayerEntity
class_name ServerEntity

onready var message_controller:MessageController = MessageController.new()
onready var timer:Timer = Timer.new()
onready var health_add_timer: Timer = Timer.new()
onready var spawn
onready var physics_native_socket = null

var socket:ClientWebSocket
var physics_socket:RustSocket
#var requested_dest = false
var last_request = null
var destination:Destination = Destination.new()
var destinations_active:bool
var gravity_active:bool
var epsilon = 3
var isSubbed:bool = false
var is_npc:bool = false
var queued_teleports = []
var queued_input = Vector3()
var last_pos:Vector3 #used for lv
var lv:Vector3

func _ready():
	destination.is_empty = true
	spawn = body.global_transform.origin
	self.add_child(message_controller)
	self.movement.body_ref = body
	self.add_child(timer)
	timer.connect("timeout",self,"check_destinations")
	timer.start(rand_range(0,2))
	init_sockets()
	
func init_sockets():
	socket = ServerNetwork.get(client_id)
	assert(socket != null)
	physics_socket = ServerNetwork.get_physics(client_id)
	assert(physics_socket != null)
	self.movement.physics_socket = physics_socket

#func add_health():
#	socket.add_health(id,10)
	
	
func check_destinations():
	socket.get_next_destination(id)
	
func check_dir():
	physics_socket.get_dir_physics(id)

func apply_direction(vec):
	if vec != null:
		dir.x = vec[0]
		dir.y = vec[1]
		dir.z = vec[2]
		var max_speed = movement.get_max_speed()
		#proc is needed to ensure network doesn't get clogged in a loop when trying to reset speed
		#todo remove this entirely in favor of physics server managing this
		dir = (dir.normalized() * min(dir.length(),max_speed) * int(max_speed != null))
		if proc %2 == 0:
			#print("direction ", Vector3(x,y,z))
			proc = 0
			#physics_socket.set_dir_physics(id,dir)
			physics_native_socket.send_direction(id,dir.x,dir.y,dir.z)
		proc += 1
		if (not destinations_active) or gravity_active:
			movement.entity_set_direction(dir)

var proc:int = 0
var dir:Vector3 = Vector3()
func default_handle_message(msg,_delta_accum):
	match msg:
		{'typ':var typ,'id':var id,'vec' : [var x , var y , var z]}:
			match typ:
				'Dir':
					dir.x = x
					dir.y = y
					dir.z = z
					var max_speed = movement.get_max_speed()
					#proc is needed to ensure network doesn't get clogged in a loop when trying to reset speed
					#todo remove this entirely in favor of physics server managing this
					dir = (dir.normalized() * min(dir.length(),max_speed) * int(max_speed != null))
					if proc %2 == 0:
						#print("direction ", Vector3(x,y,z))
						proc = 0
						physics_socket.set_dir_physics(id,dir)
						#physics_native_socket.send_direction(id,dir.x,dir.y,dir.z)
					proc += 1
					if (not destinations_active) or gravity_active:
						movement.entity_set_direction(dir)
				'Input':
					assert(false)
					queued_input.x = x
					queued_input.y = y
					queued_input.z = z
		{'GravityActive':{'id': var _id, 'is_active':var active}}:
			print_debug("gravity active " , active)
			gravity_active = bool(active)
			if destinations_active and not gravity_active:
				#physics_socket.lock_input_physics(id)
				print_debug("input locked " )
			else:
				#physics_socket.unlock_input_physics(id)
				print_debug("input unlocked")
		{'DestinationsActive':{'id': var _id, 'is_active':var active}}:
			print_debug("destinations active", active)
			destinations_active = bool(active)
			if destinations_active and not gravity_active:
				#physics_socket.lock_input_physics(id)
				print_debug("input locked")
			else:
				#physics_socket.unlock_input_physics(id)
				print_debug("input unlocked")
		{'NoInput':{'id':var _id}}:
			assert(false)
			movement.entity_stop(body)
		{'SET_GLOB_LOCATION':{'id':id,'location':var location}}:
			assert(false)
			body.global_transform.origin = location
		{'NextDestination':{'id': var _id, 'destination': {'uuid':var uuid, 'dest_type':var dest_type, 'location':[var x, var y , var z] , 'radius': var radius}}}:
			destination.location = Vector3(x,y,z)
			destination.type = dest_type
			destination.radius = radius
			destination.uuid = uuid
			destination.is_empty = false
		{'TeleportToNext':{'id':var _id, 'location':[var x, var y ,var z]}}:
			queued_teleports.push_front(Vector3(x,y,z))
		{'NoLocation':{'id':var _id}}:
			destination.is_empty = true
			destination.type = 'Empty'
		_:
			print("No server entity handler for " , msg)
			pass
	pass
	
	
func update_lv_internal(body,delta):
	if last_pos != null:
		lv = (body.global_transform.origin - last_pos)/delta
	last_pos = body.global_transform.origin
	
func get_lv() -> Vector3:
	if lv != null:
		return lv			
	else:
		return Vector3.ZERO
		
func default_physics_process2(delta):
	physics_native_socket.get_direction(id)
	apply_direction(physics_native_socket.cached_direction())
	update_lv_internal(body,delta)
	movement.entity_set_max_speed(DataCache.cached(id,'speed'))
	movement.entity_move_by_direction(delta,body)
	var should_tele:bool = body is KinematicBody and destinations_active
	if !queued_teleports.empty():
		var t = queued_teleports.pop_front()
		var dir = (t - body.global_transform.origin)
		body.translate(dir * int(should_tele))
	match destination.type:
		'Empty':
			queued_teleports.pop_front()
			pass
		'{WAYPOINT:{}}':
			movement.entity_move_by_gravity(
				id,delta * int(gravity_active) * int(destinations_active) * int(!destination.is_empty),
				destination.location,
				body
			)
			movement.entity_move(
				delta * int(!gravity_active)* int(destinations_active)* int(!destination.is_empty),
				destination.location,
				body
			)
		'{TELEPORT:{}}':
			movement.entity_move_by_gravity(
				id,delta * int(gravity_active) * int(destinations_active) * int(!destination.is_empty),
				destination.location,
				body
			)
			movement.entity_move(
				delta * int(!gravity_active)* int(destinations_active)* int(!destination.is_empty),
				destination.location,
				body
			)
		"{GRAVITY_BIND:{}}":
			movement.entity_move_by_gravity(id,delta,destination.location,body)
		_:
			print_debug("no handler found for destination with type ", destination.type)
	#physics_socket.set_location_physics(id,body.global_transform.origin)
	physics_native_socket.send_location(id,body.global_transform.origin.x,body.global_transform.origin.y,body.global_transform.origin.z)


#default physics process for all server entities
func default_physics_process(delta):
	physics_socket.get_dir_physics(id)
	update_lv_internal(body,delta)
	movement.entity_set_max_speed(DataCache.cached(id,'speed'))
	movement.entity_move_by_direction(delta,body)
	var should_tele:bool = body is KinematicBody and destinations_active
	if !queued_teleports.empty():
		var t = queued_teleports.pop_front()
		var dir = (t - body.global_transform.origin)
		body.translate(dir * int(should_tele))
	match destination.type:
		'Empty':
			queued_teleports.pop_front()
			pass
		'{WAYPOINT:{}}':
			movement.entity_move_by_gravity(
				id,delta * int(gravity_active) * int(destinations_active) * int(!destination.is_empty),
				destination.location,
				body
			)
			movement.entity_move(
				delta * int(!gravity_active)* int(destinations_active)* int(!destination.is_empty),
				destination.location,
				body
			)
		'{TELEPORT:{}}':
			movement.entity_move_by_gravity(
				id,delta * int(gravity_active) * int(destinations_active) * int(!destination.is_empty),
				destination.location,
				body
			)
			movement.entity_move(
				delta * int(!gravity_active)* int(destinations_active)* int(!destination.is_empty),
				destination.location,
				body
			)
		"{GRAVITY_BIND:{}}":
			movement.entity_move_by_gravity(id,delta,destination.location,body)
		_:
			print_debug("no handler found for destination with type ", destination.type)
	physics_socket.set_location_physics(id,body.global_transform.origin)

func _process(delta):
	if !isSubbed:
		socket.get_physical_stats(id)
		isSubbed = true
	
