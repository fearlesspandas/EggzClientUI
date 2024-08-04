extends KinematicBody

#nodes that will 'extened' PhysicalPlayerEntity simply
export var mesh_instance:Resource
onready var mesh
onready var client_player_entity:ClientPlayerEntity = get_parent()
func _ready():
	mesh = load(mesh_instance.resource_path).instance()
	
	self.add_child(mesh)
	if !client_player_entity.is_npc:
		self.set_collision_layer_bit(10,true)
		self.set_collision_mask_bit(10,true)
	pass # Replace with function body.
