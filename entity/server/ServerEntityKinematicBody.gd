extends KinematicBody

# Called when the node enters the scene tree for the first time.
onready var parent:ServerEntity = get_parent()
func _ready():
	if !parent.is_npc:
		self.set_collision_layer_bit(10,true)
		self.set_collision_layer_bit(0,true)
		self.set_collision_mask_bit(10,true)
		self.set_collision_mask_bit(0,true)
