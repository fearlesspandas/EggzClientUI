extends StaticBody


# Declare member variables here. Examples:
# var a = 2
# var b = "text"


# Called when the node enters the scene tree for the first time.
func _ready():
	self.set_collision_layer_bit(EntityConstants.SERVER_TERRAIN_COLLISION_LAYER,true)
	self.set_collision_mask_bit(EntityConstants.SERVER_TERRAIN_COLLISION_LAYER,true)

# Called every frame. 'delta' is the elapsed time since the previous frame.
#func _process(delta):
#	pass
