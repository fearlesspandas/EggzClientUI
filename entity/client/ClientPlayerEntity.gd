extends PhysicalPlayerEntity

class_name ClientPlayerEntity
#clientplayerentity is currentlyu just a message controller + an entity resource that will be assumed to have
#a child node PhysicalPlayerEntity which will contain a physics body depending on implemenmtation

onready var message_controller:MessageController = MessageController.new()
var isSubbed = false
func _ready():
	self.add_child(message_controller)
	pass # Replace with function entity.
func getSocket() -> ClientWebSocket:
	#print("entity socket",id)
	var res = ServerNetwork.get(client_id)
	if res != null and !res.connected:
		return null
	else:
		return res 
func _physics_process(delta):
	var socket = getSocket()
	#if !isSubbed and socket != null and socket.connected:
	#	socket.location_subscribe(id)
	#	print("sent subscription ",id)
	#	isSubbed = true
	if socket!= null:
		socket.getGlobLocation(id)
	#for clientPlayerEntities we only want to react to serverside data
	#only players produce messages, generically we don't want to do this
	#self.global_transform.origin = body.global_transform.origin
	#if getSocket() != null:
		#getSocket().getGlobLocation(id)
	#entity.move_and_collide(-dir)	
	pass
func _handle_message(msg,delta_accum):
	match msg:
		{'Location':{'id':id,'location':[var x , var y , var z]}}:
			var loc = Vector3(x,y,z)
			#print("setting clientside location:",loc)
			var diff:Vector3 = body.global_transform.origin - loc
			
			movement.entity_move(delta_accum,loc,body)
			pass
		_ :
			pass
