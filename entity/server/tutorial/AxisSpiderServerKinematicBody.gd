extends KinematicBody

class_name AxisSpiderServerKinematicBody

onready var parent = get_parent()

onready var top = find_node("TopLegs")
onready var bottom = find_node("BottomLegs")

func _ready():
	assert(top != null)
	assert(bottom != null)

	for ch in top.get_children():
		self.add_collision_exception_with(ch)
		self.set_collision_layer_bit(EntityConstants.SERVER_TERRAIN_COLLISION_LAYER,false)
		self.set_collision_mask_bit(EntityConstants.SERVER_TERRAIN_COLLISION_LAYER,false)

	for ch in bottom.get_children():
		self.add_collision_exception_with(ch)
		self.set_collision_layer_bit(EntityConstants.SERVER_TERRAIN_COLLISION_LAYER,false)
		self.set_collision_mask_bit(EntityConstants.SERVER_TERRAIN_COLLISION_LAYER,false)

	
