extends PhysicalPlayerEntity

class_name ClientPlayerEntity
#clientplayerentity is currentlyu just a message controller + an entity resource that will be assumed to have
#a child node PhysicalPlayerEntity which will contain a physics body depending on implemenmtation

onready var message_controller:MessageController = MessageController.new()
func _ready():
	self.add_child(message_controller)
	pass # Replace with function entity.

func _physics_process(delta):
	#for clientPlayerEntities we only want to react to serverside data
	#only players produce messages, generically we don't want to do this
	#self.global_transform.origin = body.global_transform.origin
	ServerNetwork.getGlobLocation(id)
	#entity.move_and_collide(-dir)	

func _handle_message(msg,delta_accum):
	match msg:
		{'Location':{'id':id,'location':[var x , var y , var z]}}:
			print("setting clientside location")
			var loc = Vector3(x,y,z)
			var diff:Vector3 = body.global_transform.origin - loc
			if diff.length() > 10:
				body.global_transform.origin = loc
			pass
		_ :
			pass
