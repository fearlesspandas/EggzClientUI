extends Node
class_name GravityBox

onready var ref = load("res://native_lib/GravityBox.gdns").new()

onready var parent = get_parent()
func _ready():
	ref.connect("apply_gravity",ScheduledTransforms.ref,"add_gravity")
	parent.add_child(ref)

