extends RigidBody

class_name ServerBody

func _ready():
	
func _integrate_forces(state:PhysicsDirectBodyState):
	state.add_central_force()
