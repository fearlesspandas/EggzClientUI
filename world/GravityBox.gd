extends Node
class_name GravityBox

onready var ref = load("res://native_lib/GravityBox.gdns").new()

onready var parent = get_parent()
var id

var captured_ids = {}
func _ready():
	ref.connect("apply_gravity",ScheduledTransforms.ref,"add_gravity")

	GlobalSignalsServer.connect("gravity_captured",self,"add_captured")
	GlobalSignalsServer.connect("gravity_released",self,"remove_captured")

	ref.connect("entered_unaffected",GlobalSignalsServer,"gravity_captured")
	ref.connect("exited_unaffected",GlobalSignalsServer,"gravity_released")

	parent.add_child(ref)


func set_id(id:String):
	self.id = id
	ref.set_id(id)

func add_captured(id,terrain_id):
	if !terrain_id == self.id:
		ref.add_captured(id)

func remove_captured(id,terrain_id):
	if !terrain_id == self.id:
		ref.remove_captured(id)

