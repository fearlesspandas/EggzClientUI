extends NonPlayerControlledEntity

class_name ProwlerEntity

func _physics_process(delta):
	self.default_physics_process(delta)


func _handle_message(msg,delta_accum):
	self.default_handle_message(msg,delta_accum)
