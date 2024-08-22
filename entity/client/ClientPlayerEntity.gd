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
	pass
	
func getSocket() -> ClientWebSocket:
	#print("entity socket",id)
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
var mod = 4
func _process(delta):
	movement.entity_move_by_direction(delta,body)
	if proc % mod == 0:
		get_direction()
		proc = 0
	if proc % mod == 2:
		get_location()
	proc += 1
	
func poll_physics():
	if physics_socket.connected:
		physics_socket.get_location_physics(id)
		
func _handle_message(msg,delta_accum):
	match msg:
		[var x,var y,var z]:
			#if not is_npc:
				#print("client player entity received location ", Vector3(x,y,z))
			movement.entity_move(delta_accum,Vector3(x,y,z),body)
			
			pass
		{'Dir':{'id':var id, 'vec':[var x, var y , var z]}}:
			movement.entity_set_direction(Vector3(x,y,z))
		{'HealthSet':{'id':var id,'value':var value}}:
			set_health(value)
		#deprecated
		{'Location':{'id':id,'location':[var x , var y , var z]}}:
			var loc = Vector3(x,y,z)
			#print("setting clientside location:",loc)
			var diff:Vector3 = body.global_transform.origin - loc
			
			#movement.entity_move(delta_accum,loc,body)
			pass
		_ :
			print_debug("no handler found for msg " , msg)
