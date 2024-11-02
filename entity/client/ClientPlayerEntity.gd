extends PhysicalPlayerEntity

class_name ClientPlayerEntity
#clientplayerentity is currentlyu just a message controller + an entity resource that will be assumed to have
#a child node PhysicalPlayerEntity which will contain a physics body depending on implemenmtation

onready var message_controller:MessageController = MessageController.new()
onready var username:Username = Username.new()
onready var health:HealthDisplay = HealthDisplay.new()
onready var physics_native_socket = null

var isSubbed = false
var is_npc = false
var physics_socket:RustSocket
var socket : ClientWebSocket
var mod = 2
var radius = 0

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

func default_physics_process2(delta,mod = 2):
	physics_native_socket.get_location(id)
	var l = physics_native_socket.cached_location()
	if l != null:
		loc.x = l[0]
		loc.y = l[1]
		loc.z = l[2]
		movement.entity_move(delta,loc,body)


var proc = 0
#basic location polling
func default_physics_process(delta,mod = 2):
	if mod == 2:
		get_location()
	if proc % mod == 0:
		get_location()
		proc = 0
	if proc % mod == ceil(mod/2):
		get_location()
	proc += 1
	
func poll_physics():
	if physics_socket.connected:
		physics_socket.get_location_physics(id)
		
var loc:Vector3 = Vector3()
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
					movement.entity_set_direction(Vector3(x,y,z))
		{'HealthSet':{'id':var id,'value':var value}}:
			set_health(value)
		_ :
			print_debug("no handler found for msg " , msg)

func default_update_player_location(location):
	assert(radius > 0)
	#could microoptimize this further by just inlining and not creating any variables
	var less_than_radius = 2 * int((location - self.body.global_transform.origin).length() < radius) 
	var radius_2         = 4 * int((location - self.body.global_transform.origin).length() < 2 * radius)
	var radius_4         = 8 * int((location - self.body.global_transform.origin).length() < 4 * radius)
	var radius_8         = 16 * int((location - self.body.global_transform.origin).length() < 8 * radius)
	var radius_max       = 32 * int((location - self.body.global_transform.origin).length() > 8 * radius) 
	var t_mod = (
		2 * int(less_than_radius)
		+ 4 * int(!less_than_radius and radius_2)
		+ 8 * int(!radius_2 and radius_4)
		+ 16 * int(!radius_4 and radius_8)
		+ 32 * int(!radius_8 and radius_max)
	)
	#not sure if less reads impacts average performance between radius updates positively
	if self.mod != t_mod:
		self.mod = t_mod
	
