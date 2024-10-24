extends ClientEntityKinematicBody

#nodes that will 'extened' PhysicalPlayerEntity simply
export var mesh_instance:Resource
onready var mesh
func _ready():
	mesh = load(mesh_instance.resource_path).instance()
	self.add_child(mesh)
	if !client_player_entity.is_npc:
		print_debug("SETTING TERRAIN COLLISION FOR ", client_player_entity.id)
		#separate to PlayerEntity class
		self.set_collision_layer_bit(EntityConstants.SERVER_TERRAIN_COLLISION_LAYER,false)
		self.set_collision_mask_bit(EntityConstants.SERVER_TERRAIN_COLLISION_LAYER,false)
		self.set_collision_layer_bit(EntityConstants.CLIENT_PLAYER_COLLISION_LAYER,true)
		self.set_collision_mask_bit(EntityConstants.CLIENT_PLAYER_COLLISION_LAYER,true)
		self.set_collision_mask_bit(EntityConstants.CLIENT_NPC_COLLISION_LAYER,true)

