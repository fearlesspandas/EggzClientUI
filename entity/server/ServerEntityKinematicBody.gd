extends KinematicBody

# Called when the node enters the scene tree for the first time.
func _ready():
	self.set_collision_layer_bit(10,true)
	self.set_collision_mask_bit(10,true)
