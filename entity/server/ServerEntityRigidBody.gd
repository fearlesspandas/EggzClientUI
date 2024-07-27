extends RigidBody

class_name ServerBody
var server_entity = get_parent()

func _ready():
	self.custom_integrator = true
#func _integrate_forces(state):
#	if(server_entity.destination != null ):
#		var diff = server_entity.destination.location - self.global_transform.origin
#		if diff.length() > server_entity.epsilon:
#			var base = server_entity.destination.location - self.global_transform.origin
#			var dir = base.normalized()
#			
#			self.add_central_force(dir * 1000)
#		else:
#			#movement.entity_stop(body)
#			server_entity.destination = null
