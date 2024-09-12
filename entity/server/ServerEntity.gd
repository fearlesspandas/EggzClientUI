extends PhysicalPlayerEntity
class_name ServerEntity

onready var message_controller:MessageController = MessageController.new()
onready var timer:Timer = Timer.new()
onready var health_add_timer: Timer = Timer.new()
onready var spawn

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
	timer.start()
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

func _handle_message(msg,_delta_accum):
	match msg:
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
		{'Dir':{'id':var id, 'vec':[var x, var y , var z]}}:
			var dir = Vector3(x,y,z)
			var max_speed = movement.get_max_speed()#DataCache.cached(id,'speed')
			if gravity_active and max_speed == null :
				assert(false)
			#need this dumb fix with random so that network doesn't get clogged in a loop
			if max_speed != null and dir.length() > max_speed and rand_range(0,2) > 1:
				#print("direction ", Vector3(x,y,z))
				dir = dir.normalized() * max_speed
				physics_socket.set_dir_physics(id,dir)
			if (not destinations_active) or gravity_active:
				movement.entity_set_direction(dir)
		{'Input':{"id":var _id, "vec":[var x ,var y ,var z]}}:
			assert(false)
			queued_input = Vector3(x,y,z)
		{'SET_GLOB_LOCATION':{'id':id,'location':var location}}:
			body.global_transform.origin = location
		{'NextDestination':{'id': var _id, 'destination': {'uuid':var uuid, 'dest_type':var dest_type, 'location':[var x, var y , var z] , 'radius': var radius}}}:
			#requested_dest = false
			destination.location = Vector3(x,y,z)
			destination.type = dest_type
			destination.radius = radius
			destination.uuid = uuid
			destination.is_empty = false
		{'TeleportToNext':{'id':var _id, 'location':[var x, var y ,var z]}}:
			print_debug("teleporting " , x,y,z)
			queued_teleports.push_front(Vector3(x,y,z))
		{'NoLocation':{'id':var _id}}:
			destination.is_empty = true
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
	physics_socket.get_dir_physics(id)
	update_lv_internal(body,delta)
	movement.entity_set_max_speed(DataCache.cached(id,'speed'))
	movement.entity_move_by_direction(delta,body)
	if !queued_teleports.empty() and body is KinematicBody and destinations_active:
		var t = queued_teleports.pop_front()
		var dir = (t - body.global_transform.origin)#.normalized()
		body.translate(dir)
	if(!destination.is_empty and destinations_active):
		#var diff = destination.location - body.global_transform.origin
		match destination.type:
			'{WAYPOINT:{}}':
				if gravity_active:
					movement.entity_move_by_gravity(id,delta,destination.location,body)
				else:
					movement.entity_move(delta,destination.location,body)
			'{TELEPORT:{}}':
				if gravity_active:
					movement.entity_move_by_gravity(id,delta,destination.location,body)
				else:
					movement.entity_move(delta,destination.location,body)
			"{GRAVITY_BIND:{}}":
				movement.entity_move_by_gravity(id,delta,destination.location,body)
			_:
				print_debug("no handler found for destination with type ", destination.type)
	else:
		queued_teleports.pop_front()
	physics_socket.set_location_physics(id,body.global_transform.origin)
	
func _process(delta):
	if !isSubbed:
		socket.get_physical_stats(id)
		isSubbed = true
	
