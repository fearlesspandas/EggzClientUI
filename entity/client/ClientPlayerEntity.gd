extends PhysicalPlayerEntity

class_name ClientPlayerEntity
#clientplayerentity is currentlyu just a message controller + an entity resource that will be assumed to have
#a child node PhysicalPlayerEntity which will contain a physics body depending on implemenmtation

onready var message_controller:MessageController = MessageController.new()
onready var username:Username = Username.new()
onready var timer:Timer = Timer.new()
var isSubbed = false
var is_npc = false
var physics_socket:RustSocket
var socket : ClientWebSocket

func _ready():
	username.init_id()
	Subscriptions.subscribe(username.id,id)
	self.add_child(username)
	self.add_child(message_controller)
	socket = ServerNetwork.get(client_id)
	assert(socket != null)
	physics_socket = ServerNetwork.get_physics(client_id)
	assert(physics_socket != null)
	if is_npc:
		timer.connect("timeout",self,"poll_physics")
		timer.wait_time = 0.25
		self.add_child(timer)
		timer.start()
	pass # Replace with function entity.
	
func getSocket() -> ClientWebSocket:
	#print("entity socket",id)
	var res = ServerNetwork.get(client_id)
	if res != null and !res.connected:
		return null
	else:
		return res 
func _process(delta):
	if !is_npc:
		poll_physics()
		
func poll_physics():
	if physics_socket.connected:
		physics_socket.get_location_physics(id)
		
func _handle_message(msg,delta_accum):
	match msg:
		[var x,var y,var z]:
			#print_debug("Entity received physics, " , x,y,z)
			movement.entity_move(delta_accum,Vector3(x,y,z),body)
			pass
		{'Location':{'id':id,'location':[var x , var y , var z]}}:
			var loc = Vector3(x,y,z)
			#print("setting clientside location:",loc)
			var diff:Vector3 = body.global_transform.origin - loc
			
			#movement.entity_move(delta_accum,loc,body)
			pass
		_ :
			pass
