extends Node

class_name Movement

onready var this = self
var physics_socket:RustSocket

	
func entity_teleport(location:Vector3,body):
	if self.has_method("teleport"):
		this.teleport(location,body)
		
func entity_move(delta,location:Vector3,body):
	if self.has_method("move"):
		this.move(delta,location,body)
	else:
		pass

func entity_move_by_gravity(id,delta,location:Vector3,body):
	if self.has_method("move_by_gravity"):
		this.move_by_gravity(id,delta,location,body)
	else:
		pass

func entity_stop(body):
	if self.has_method("stop"):
		this.stop(body)
	else:
		pass

func entity_set_direction(direction:Vector3):
	if self.has_method("set_direction"):
		this.set_direction(direction)

func entity_apply_direction(direction:Vector3):
	if self.has_method("apply_direction"):
		this.apply_direction(direction)

func entity_apply_vector(delta,vector:Vector3,body):
	if self.has_method("apply_vector"):
		this.apply_vector(delta,vector,body)
	else:
		pass

func entity_move_by_direction(delta,body):
	if self.has_method("move_by_direction"):
		this.move_by_direction(delta,body)
	else:
		pass
		
func entity_get_direction() -> Vector3:
	if self.has_meta("get_direction"):
		return this.get_direction()
	else:
		return Vector3.ZERO

func entity_get_lv(body) -> Vector3:
	if self.has_method("get_lv"):
		return this.get_lv(body)
	else:
		return Vector3.ZERO

func entity_set_max_speed(max_speed):
	if self.has_method("set_max_speed"):
		this.set_max_speed(max_speed)
	else:
		pass

