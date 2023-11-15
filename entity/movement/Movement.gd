extends Node

class_name Movement

onready var this = self
func _ready():
	pass

func entity_move(delta,location:Vector3,body:KinematicBody):
	if self.has_method("move"):
		this.move(delta,location,body)
	else:
		pass
