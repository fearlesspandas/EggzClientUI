extends PhysicalPlayerEntity
class_name ServerEntity

onready var message_controller:MessageController = MessageController.new()
onready var timer:Timer = Timer.new()
onready var spawn
var socket:ClientWebSocket
var physics_socket:RustSocket
var requested_dest = false
var timeout = 10
var last_request = null
var destination:Destination = null
var epsilon = 3
var isSubbed:bool = false
var is_npc:bool = false

func _ready():
	spawn = body.global_transform.origin
	self.add_child(message_controller)
	
	
	self.movement.body_ref = body
	
	if is_npc:
		timer.connect("timeout",self,"npc_polling")
		timer.wait_time = 0.5
		self.add_child(timer)
		timer.start()
	else: 
		timer.connect("timeout",self,"timer_polling")
		timer.wait_time = 0.5
		self.add_child(timer)
		timer.start()
	init_sockets()
	
func init_sockets():
	socket = ServerNetwork.get(client_id)
	assert(socket != null)
	physics_socket = ServerNetwork.get_physics(client_id)
	assert(physics_socket != null)
	
func timer_polling():
	var lv = movement.entity_get_lv(body)
	socket.set_lv(id,movement.entity_get_lv(body))
	
func npc_polling():
	socket.get_next_destination(id)
func _handle_message(msg,delta_accum):
	match msg:
		{'NoInput':{'id':var id}}:
			movement.entity_stop(body)
			#movement.entity_apply_vector(delta_accum,Vector3.ZERO,body)
		{'Input':{"id":var id, "vec":[var x ,var y ,var z]}}:
			#print_debug("got input" , x,y,z)
			movement.entity_apply_vector(delta_accum,Vector3(x,y,z),body)
		{'SET_GLOB_LOCATION':{'id':id,'location':var location}}:
			body.global_transform.origin = location
		{'NextDestination':{'id': var id, 'destination': {'dest_type':var dest_type, 'location':[var x, var y , var z] , 'radius': var radius}}}:
			requested_dest = false
			destination = Destination.new()
			destination.location = Vector3(x,y,z)
			destination.type = dest_type
			destination.radius = radius
		{'NoLocation':{'id':var id}}:
			destination = null
		_:
			print("No server entity handler for " , msg)
			pass
	pass
	
func freeze():
	body.global_transform.origin = spawn
	
func _physics_process(delta):
	#movement.entity_set_max_speed(DataCache.cached(id,'max_speed'))
	self.global_transform.origin = body.global_transform.origin
	physics_socket.get_input_physics(id)
	physics_socket.set_location_physics(id,body.global_transform.origin)
	if !is_npc:
		socket.get_next_destination(id)
	movement.entity_set_max_speed(DataCache.cached(id,'max_speed'))
	if(destination != null ):
		var diff = destination.location - body.global_transform.origin
		match destination.type:
			'{WAYPOINT:{}}':
				if diff.length() > epsilon:
					movement.entity_move(delta,destination.location,body)
				else:
					#movement.entity_stop(body)
					destination = null
			"GRAVITY_BIND":
				if diff.length() > epsilon:
					movement.entity_move_by_gravity(delta,destination.location,body)
				else:
					#movement.entity_stop(body)
					destination = null
					
			_:
				print_debug("no handler found for destination with type ", destination.type)

func _process(delta):
	if !isSubbed:
		#socket.input_subscribe(id)
		var query = PayloadMapper.get_physical_stats(id)
		#if socket!=null:
		socket.subscribe_general(query)
		isSubbed = true
		#print_debug("subbing to input for id", id)


func _input(event):
	if event is InputEventKey and event.is_action_pressed("alt",true):
		movement.entity_stop(body)
	
