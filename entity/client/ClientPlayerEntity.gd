extends PhysicalPlayerEntity

class_name ClientPlayerEntity
#clientplayerentity is currentlyu just a message controller + an entity resource that will be assumed to have
#a child node PhysicalPlayerEntity which will contain a physics body depending on implemenmtation

onready var message_controller:MessageController = MessageController.new()
onready var username:Username = Username.new()
onready var health:HealthDisplay = HealthDisplay.new()

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
		
func default_handle_message(msg,delta_accum):
	match msg:
		[var x,var y,var z]:
			movement.entity_move(delta_accum,Vector3(x,y,z),body)
			pass
		{'Dir':{'id':var id, 'vec':[var x, var y , var z]}}:
			movement.entity_set_direction(Vector3(x,y,z))
		{'HealthSet':{'id':var id,'value':var value}}:
			set_health(value)
		_ :
			print_debug("no handler found for msg " , msg)

func default_update_player_location(location):
	assert(radius > 0)
	if (location - self.body.global_transform.origin).length() < radius:	
		self.mod = 4
	elif (location - self.body.global_transform.origin).length() < 2 * radius:
		self.mod = 4
	elif (location - self.body.global_transform.origin).length() < 4 * radius:
		self.mod = 6
	elif (location - self.body.global_transform.origin).length() < 8 * radius:
		self.mod = 16
	else:
		self.mod = 32
