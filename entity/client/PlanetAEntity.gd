extends ClientPlayerEntity
class_name PlanetAEntity

func _physics_process(delta):
	default_physics_process(delta,2)

func _handle_message(msg,delta_accum):
	self.default_handle_message(msg,delta_accum)

func init_with_id(id,client_id):
	id.erase(16,64)
	self.id = id
	self.client_id = client_id

