extends StaticBody
onready var parent = get_parent()

func _ready():
	self.input_ray_pickable = true
	self.set_collision_layer_bit(0,true)
	connect("mouse_entered",self,"glow")
	connect("input_event",self,"handle_input")
	
func glow():
	assert(false)

func handle_input(camera,event,position,normal,shape_idx):
	assert(false)
