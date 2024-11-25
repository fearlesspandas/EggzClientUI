extends NonPlayerControlledEntity

class_name ProwlerEntity

func _ready():
	self.radius = 32
	GlobalSignalsClient.connect("player_position",self,"update_player_location")

func _physics_process(delta):
	self.default_physics_process(delta,self.mod)


func _handle_message(msg,delta_accum):
	self.default_handle_message(msg,delta_accum)


func update_player_location(location):
	if (self.global_transform.origin - location).length() > self.radius * 32:
		self.body.visible = false
		self.username.visible = false
	else:
		self.body.visible = true
		self.username.visible = true
	default_update_player_location(location)
		
		
