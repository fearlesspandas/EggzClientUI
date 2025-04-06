extends KinematicBody

export var mesh_resource:Resource

func _ready():
	var mesh = load(mesh_resource.resource_path).instance()
	self.add_child(mesh)
	$CollisionShape.disabled = true
	
	self.set_collision_layer_bit(EntityConstants.SERVER_TERRAIN_COLLISION_LAYER,false)
	self.set_collision_mask_bit(EntityConstants.SERVER_TERRAIN_COLLISION_LAYER,false)
	self.set_collision_layer_bit(EntityConstants.CLIENT_PLAYER_COLLISION_LAYER,false)
	self.set_collision_mask_bit(EntityConstants.CLIENT_PLAYER_COLLISION_LAYER,false)
