extends PhysicalPlayerEntity
class_name ServerEntity

onready var message_controller:MessageController = MessageController.new()
onready var timer:Timer = Timer.new()
onready var health_add_timer: Timer = Timer.new()
onready var spawn
onready var physics_native_socket = null
onready var physics_native_shared_socket = SharedRuntimeEnv.physics_native_shared_socket

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

var debug_mesh = null

var socket_mode = ServerTerminalGlobalSignals.SocketMode.Native
func _ready():
	destination.is_empty = true
	spawn = body.global_transform.origin
	self.add_child(message_controller)
	self.movement.body_ref = body
	self.add_child(timer)
	timer.connect("timeout",self,"check_destinations")
	timer.start(rand_range(0,2))
	init_sockets()
	physics_native_shared_socket.add_entity_to_queue(id)
	ServerTerminalGlobalSignals.connect("entities_add_mesh",self,"add_mesh")
	ServerTerminalGlobalSignals.connect("entities_remove_mesh",self,"remove_mesh")
	ServerTerminalGlobalSignals.connect("set_entity_socket_mode",self,"set_socket_mode_if_entity")
	ServerTerminalGlobalSignals.connect("set_all_entity_socket_mode",self,"set_socket_mode")
	ServerTerminalGlobalSignals.connect("request_data",self,"send_requested_data")
	ServerTerminalGlobalSignals.connect("set_health",self,"terminal_set_health")
	ServerTerminalGlobalSignals.connect("give_ability",self,"terminal_give_ability")
	
func send_requested_data(data_type):
	match data_type:
		ServerTerminalGlobalSignals.StreamDataType.socket_mode:
			ServerTerminalGlobalSignals.add_input_data(self.id + "_server_socket_mode" ,str(socket_mode))
		ServerTerminalGlobalSignals.StreamDataType.linear_velocity:
			ServerTerminalGlobalSignals.add_graph_data(self.id + "_server_lv" ,movement.dir.length())
		ServerTerminalGlobalSignals.StreamDataType.global_position:
			ServerTerminalGlobalSignals.add_graph_data(self.id + "_server_position" ,body.global_transform.origin.length())
		ServerTerminalGlobalSignals.StreamDataType.requests_sent:
			ServerTerminalGlobalSignals.add_graph_data(self.id + "_server_req_sent" ,float(physics_native_shared_socket.num_sent(id)))
		ServerTerminalGlobalSignals.StreamDataType.responses_received:
			ServerTerminalGlobalSignals.add_graph_data(self.id + "_server_resp_recv" ,float(physics_native_shared_socket.num_received(id)))
		ServerTerminalGlobalSignals.StreamDataType.request_response_delta:
			ServerTerminalGlobalSignals.add_graph_data(self.id + "_server_resp_delta" ,float(physics_native_shared_socket.num_sent(id) - physics_native_shared_socket.num_received(id)))

func set_socket_mode_if_entity(id,mode):
	if id == self.id or id == "SERVER":
		self.socket_mode = mode

func set_socket_mode(mode):
	self.socket_mode = mode

func terminal_set_health(id,health):
	if id == self.id:
		socket.remove_health(self.id,health)

func terminal_give_ability(id,health):
	if id == self.id:
		socket.add_item(self.id,health)

func init_sockets():
	socket = ServerNetwork.get(client_id)
	assert(socket != null)
	physics_socket = ServerNetwork.get_physics(client_id)
	assert(physics_socket != null)
	self.movement.physics_socket = physics_socket

	
func add_mesh():
	var mesh_instance = MeshInstance.new()
	var mesh = SphereMesh.new()
	mesh.radius = 2 
	mesh.height = 1
	mesh_instance.mesh = mesh
	body.add_child(mesh_instance)
	debug_mesh = mesh_instance

func remove_mesh():
	body.remove_child(debug_mesh)
	debug_mesh.call_deferred("free")

	
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
			physics_native_shared_socket.send_direction(id,dir.x,dir.y,dir.z)
		proc += 1
		if (not destinations_active) or gravity_active:
			movement.entity_set_direction(dir)

#DEFAULT_PHYSICS##################
func default_physics_process(delta,mod = 2):
	match socket_mode:
		ServerTerminalGlobalSignals.SocketMode.Native:
			default_physics_process_native(delta)
		ServerTerminalGlobalSignals.SocketMode.GodotClient:
			default_physics_process_godot(delta)
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
					proc += 1
					if (not destinations_active) or gravity_active:
						movement.entity_set_direction(dir)
				'Input':
					assert(false)
					queued_input.x = x
					queued_input.y = y
					queued_input.z = z
		{'GravityActive':{'id': var _id, 'is_active':var active}}:
			gravity_active = bool(active)
		{'DestinationsActive':{'id': var _id, 'is_active':var active}}:
			destinations_active = bool(active)
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
		{'FieldCleared':{'entity_id':var id}}:
			print_debug("Received Field Clear for id ", id)
		_:
			assert(false)
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
		
#DEFAULT_PHYSICS_NATIVE###########
#default physics process running native multithreaded sockets
#Every client entity has a native socket process
#Scales better but is technically less performant than 
# the standard godot socket handling.
#This is used mainly to guarentee that lots of entities
# do not block other cpu actions.
#When this does happen using the single threaded socket
# handling network traffic becomes a frame bottleneck 
# (probably due to message routing but not currently certain)
func default_physics_process_native(delta):
	physics_native_shared_socket.request_direction(id)
	apply_direction(physics_native_shared_socket.get_direction(id))
	update_lv_internal(body,delta)
	movement.entity_set_max_speed(DataCache.cached(id,'speed'))
	movement.entity_move_by_direction(delta,body)
	var should_tele:bool = body is KinematicBody #####and destinations_active
	if !queued_teleports.empty():
		var t = queued_teleports.pop_front()
		var dir = (t - body.global_transform.origin)
		body.translate(dir * int(should_tele))
	if !destinations_active or destination.is_empty:
		physics_socket.set_location_physics(id,body.global_transform.origin)
		return

	if !gravity_active:
		match destination.type:
			'Empty':
				queued_teleports.pop_front()
				pass
			'{WAYPOINT:{}}':
				movement.entity_move(
					delta ,
					destination.location,
					body
				)
			'{TELEPORT:{}}':
				movement.entity_move(
					delta,
					destination.location,
					body
				)
			"{GRAVITY_BIND:{}}":
				movement.entity_move_by_gravity(id,delta,destination.location,body)
			_:
				assert(false)

	match destination.type:
		'Empty':
			queued_teleports.pop_front()
			pass
		_:
			movement.entity_move_by_gravity(
				id,
				delta,
				destination.location,
				body
			)
	physics_native_shared_socket.send_location(id,body.global_transform.origin.x,body.global_transform.origin.y,body.global_transform.origin.z)

######################################
#DEFAULT_PHYSICS_GODOT################
#default physics process running non-native single threaded sockets
#returning data is handled by default_handle_message
#basic location polling
func default_physics_process_godot(delta):
	physics_socket.get_dir_physics(id)
	update_lv_internal(body,delta)
	movement.entity_set_max_speed(DataCache.cached(id,'speed'))
	movement.entity_move_by_direction(delta,body)
	var should_tele:bool = body is KinematicBody ###and destinations_active
	if !queued_teleports.empty():
		var t = queued_teleports.pop_front()
		var dir = (t - body.global_transform.origin)
		body.translate(dir * int(should_tele))
	if !destinations_active or destination.is_empty:
		physics_socket.set_location_physics(id,body.global_transform.origin)
		return

	if !gravity_active:
		match destination.type:
			'Empty':
				queued_teleports.pop_front()
				pass
			'{WAYPOINT:{}}':
				movement.entity_move(
					delta ,
					destination.location,
					body
				)
			'{TELEPORT:{}}':
				movement.entity_move(
					delta,
					destination.location,
					body
				)
			"{GRAVITY_BIND:{}}":
				movement.entity_move_by_gravity(id,delta,destination.location,body)
			_:
				assert(false)

	match destination.type:
		'Empty':
			queued_teleports.pop_front()
			pass
		_:
			movement.entity_move_by_gravity(
				id,
				delta,
				destination.location,
				body
			)
	physics_socket.set_location_physics(id,body.global_transform.origin)

func _process(delta):
	if !isSubbed:
		socket.get_physical_stats(id)
		isSubbed = true
	
