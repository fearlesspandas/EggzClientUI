extends Node

class_name Movement

onready var this = self
func _ready():
	pass

func entity_move(delta,location:Vector3,body):
	if self.has_method("move"):
		this.move(delta,location,body)
	else:
		pass

func entity_stop(delta,body):
	if self.has_method("stop"):
		this.stop(delta,body)
	else:
		pass

func entity_apply_vector(delta,vector:Vector3,body):
	if self.has_method("apply_vector"):
		this.apply_vector(delta,vector,body)
	else:
		pass

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

