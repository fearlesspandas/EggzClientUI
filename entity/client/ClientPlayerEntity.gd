extends PhysicalPlayerEntity

class_name ClientPlayerEntity

onready var message_controller:MessageController = MessageController.new()
onready var username:Username = Username.new()
onready var health:HealthDisplay = HealthDisplay.new()
onready var physics_native_socket = load("res://native_lib/ClientPhysicsSocket.gdns").new()


var isSubbed = false
var is_npc = false
var physics_socket:RustSocket
var socket : ClientWebSocket
var socket_mode = ClientTerminalGlobalSignals.SocketMode.Native
var mod = 2
var radius = 0

var data_stream_mode
func _ready():
	username.init_id()
	Subscriptions.subscribe(username.id,id)
	body.add_child(username)
	body.add_child(health)
	self.add_child(message_controller)
	socket = ServerNetwork.get(client_id)
	assert(socket != null)
	physics_socket = ServerNetwork.get_physics(client_id)
	assert(physics_socket != null)
	self.add_child(physics_native_socket)

	ClientTerminalGlobalSignals.connect("set_entity_socket_mode",self,"set_socket_mode_if_entity")
	ClientTerminalGlobalSignals.connect("set_all_entity_socket_mode",self,"set_socket_mode")
	ClientTerminalGlobalSignals.connect("request_data",self,"send_requested_data")


func send_requested_data(data_type):
	match data_type:
		ClientTerminalGlobalSignals.StreamDataType.socket_mode:
			ClientTerminalGlobalSignals.add_input_data(self.id + "_socket_mode" ,str(socket_mode))
		ClientTerminalGlobalSignals.StreamDataType.linear_velocity:
			ClientTerminalGlobalSignals.add_graph_data(self.id+ "_lv" ,movement.dir.length())

	
func set_socket_mode_if_entity(id,mode):
	if id == self.id:
		self.socket_mode = mode
		if self.socket_mode == ClientTerminalGlobalSignals.SocketMode.NativeProcess:
			physics_native_socket.connect("physics",self,"update_cached_physics")
		else:
			physics_native_socket.disconnect("physics",self,"update_cached_physics")

func set_socket_mode(mode):
	self.socket_mode = mode
	if self.socket_mode == ClientTerminalGlobalSignals.SocketMode.NativeProcess:
		physics_native_socket.connect("physics",self,"update_cached_physics")
	else:
		physics_native_socket.disconnect("physics",self,"update_cached_physics")
	
func getSocket() -> ClientWebSocket:
	var res = ServerNetwork.get(client_id)
	if res != null and !res.connected:
		return null
	else:
		return res 
	
func set_health(value:float):
	health.set_value(value)
	
func get_direction():
	physics_socket.get_dir_physics(id)
	
func get_location():
	physics_socket.get_location_physics(id)

###########PHYSICS################
##################################	
var last_delta = 0
var proc = 0
var loc:Vector3 = Vector3()
#DEFAULT_PHYSICS##################
func default_physics_process(delta,mod = 2):
	match socket_mode:
		ClientTerminalGlobalSignals.SocketMode.Native:
			default_physics_process_native(delta,mod)
		ClientTerminalGlobalSignals.SocketMode.NativeProcess:
			default_physics_process_native_syncd(delta,mod)
		ClientTerminalGlobalSignals.SocketMode.GodotClient:
			default_physics_process2(delta,mod)
##################################	
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
func default_physics_process_native(delta,mod = 2):
	if mod <= 2:
		physics_native_socket.get_location(id)
		physics_native_socket.get_direction(id)
	else:
		if proc%mod == 0:
			proc = 0
			physics_native_socket.get_location(id)
		elif proc%mod == ceil(mod/2):
			physics_native_socket.get_location(id)
		else:
			physics_native_socket.get_direction(id)
		proc +=1

	var l = physics_native_socket.cached_location()
	var d = physics_native_socket.cached_direction()
	last_delta = last_delta * int(loc.x != l[0] and loc.y!=l[1] and loc.z != l[2])
	last_delta += delta
	loc.x = l[0]
	loc.y = l[1]
	loc.z = l[2]
	movement.entity_move(delta,loc,body)
	loc.x = d[0]
	loc.y = d[1]
	loc.z = d[2]
	movement.set_direction(-loc)
	movement.move_by_direction(last_delta,body)
######################################
#DEFAULT_PHYSICS_GODOT################
#default physics process running non-native single threaded sockets
#returning data is handled by default_handle_message
#basic location polling
func default_physics_process2(delta,mod = 2):
	if mod == 2:
		get_location()
	if proc % mod == 0:
		get_location()
		proc = 0
	if proc % mod == ceil(mod/2):
		get_location()
	proc += 1
######################################
#DEFAULT_PHYSICS_NATIVE_SYNCHRONIZED##
var cached_loc:Vector3
#default physics native process that uses signals
func update_cached_physics(typ,vec):
	match typ:
		"location":
			last_delta = last_delta * int(cached_loc.x != vec.x and cached_loc.y!= vec.y and cached_loc.z != vec.z)
			cached_loc.x = vec.x
			cached_loc.y = vec.y
			cached_loc.z = vec.z
			movement.entity_move(last_delta,cached_loc,body)

func default_physics_process_native_syncd(delta,mod = 2):
	if mod <= 2:
		physics_native_socket.get_location(id)
	else:
		if proc%mod == 0:
			proc = 0
			physics_native_socket.get_location(id)
		if proc%mod == ceil(mod/2):
			physics_native_socket.get_location(id)
		proc +=1
######################################
#DEFAULT_HANDLE_MESSAGE###############
#default message handling for entities
#includes physics data handling for backwards compatability
# before native rust sockets were introduced.
#Messages are passed from the main ClientWebSocket for the client
# via routing in ClientEntityManagement
func default_handle_message(msg,delta_accum):
	match msg:
		{'typ':var typ,'id':var id,'vec' : [var x , var y , var z]}:
			match typ:
				"Loc":
					loc.x = x
					loc.y = y
					loc.z = z
					movement.entity_move(delta_accum,loc,body)
				"Dir":
					assert(false)
					loc.x = x
					loc.y = y
					loc.z = z
					movement.entity_set_direction(loc)
					movement.move_by_direction(last_delta,body)
		{'HealthSet':{'id':var id,'value':var value}}:
			set_health(value)
		_ :
			print_debug("no handler found for msg " , msg)

######################################
#DEFAULT_UPDATE_PLAYER_LOCATION#######
#Updates self.mod for the entity based on the current active players location
#As an entity (npc or player entity that isnt the active player) gets father
#from the active player we send less frequent location updates (the frequency of
# which is scaled by self.mod with mod <= 2 being every frame)
func default_update_player_location(location):
	assert(radius > 0)
	#could microoptimize this further by just inlining and not creating any variables
	var less_than_radius = 2 * int((location - self.body.global_transform.origin).length() < radius) 
	var radius_2         = 4 * int((location - self.body.global_transform.origin).length() < 2 * radius)
	var radius_4         = 8 * int((location - self.body.global_transform.origin).length() < 4 * radius)
	var radius_8         = 16 * int((location - self.body.global_transform.origin).length() < 8 * radius)
	var radius_16        = 32 * int((location - self.body.global_transform.origin).length() < 16 * radius)
	var radius_max       = 64 * int((location - self.body.global_transform.origin).length() > 16 * radius) 
	var t_mod = (
		2 * int(less_than_radius)
		+ 4 * int(!less_than_radius and radius_2)
		+ 8 * int(!radius_2 and radius_4)
		+ 16 * int(!radius_4 and radius_8)
		+ 32 * int(!radius_8 and radius_16)
		+ 32 * int(!radius_8 and radius_max)
	)
	#not sure if less reads impacts average performance between radius updates positively
	if self.mod != t_mod:
		self.mod = t_mod
	
