extends NPCServerEntity

class_name ProwlerServerEntity

var state = STATE.default

enum STATE{
	default,
}

func prowler_physics_process(prowler_state,delta):
	match prowler_state:
		STATE.default:
			self.default_physics_process(delta)
		_:
			print_debug("couldn't find state handler for ", state)


func _physics_process(delta):
	prowler_physics_process(self.state,delta)


func _handle_message(msg,delta_accum):
	match self.state:	
		STATE.default:
			self.default_handle_message(msg,delta_accum)
		_:
			print_debug("couldn't find state handler for ", self.state)
