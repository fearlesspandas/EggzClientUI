extends NonPlayerControlledEntity

class_name ProwlerEntity

func _ready():
	self.radius = 16
	GlobalSignalsClient.connect("player_location",self,"default_update_player_location")

func _physics_process(delta):
	self.default_physics_process(delta,self.mod)


func _handle_message(msg,delta_accum):
	self.default_handle_message(msg,delta_accum)


