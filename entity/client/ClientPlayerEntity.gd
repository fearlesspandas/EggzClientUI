extends Spatial


#clientplayerentity is currentlyu just a message controller + an entity resource that will be assumed to have
#a child node PhysicalPlayerEntity which will contain a physics body depending on implemenmtation
export var physical_entity:Resource

onready var entity = load(physical_entity.resource_path).instance()
onready var body = entity.find_node("PhysicalPlayerEntity")
onready var message_controller = find_node("MessageController")
onready var id = body.id
func _ready():
	
	self.add_child(entity)
	pass # Replace with function entity.

func _physics_process(delta):
	#for clientPlayerEntities we only want to react to serverside data
	#only players produce messages, generically we don't want to do this
	self.global_transform.origin = entity.global_transform.origin
	#entity.move_and_collide(-dir)	

func _handle_message(msg,delta_accum):
	print("entered client message handler")
	match msg:
		{'SET_GLOB_LOCATION':{'id':body.id,'location':var location}}:
			print("setting clientside location")
			body.body.global_transform.origin = location
			pass
		_ :
			pass
	print("mesasge has been handled yall we got it")
