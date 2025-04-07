extends Node
class_name CollisionBox

onready var ref = load("res://native_lib/CollisionBox.gdns").new()

onready var parent = get_parent()
func _ready():
	parent.add_child(ref)

