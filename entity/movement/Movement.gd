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
