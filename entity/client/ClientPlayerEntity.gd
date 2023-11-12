extends Spatial


#clientplayerentity is currentlyu just a message controller + an entity resource that will be assumed to have
#a child node PhysicalPlayerEntity which will contain a physics body depending on implemenmtation
export var physical_entity:Resource

onready var entity
func _ready():
	entity = load(physical_entity.resource_path).instance()
	
	self.add_child(entity)
	pass # Replace with function entity.

func _physics_process(delta):
	#for clientPlayerEntities we only want to react to serverside data
	#only players produce messages, generically we don't want to do this
	self.global_transform.origin = entity.global_transform.origin
	#entity.move_and_collide(-dir)	

func _handle_message(msg,delta_accum):
	print("mesasge has been handled yall we got it")

