extends Node
class_name Notifications

onready var ref = load("res://native_lib/Notifications.gdns").new()

onready var parent = get_parent()

func _ready():
	parent.add_child(ref)
	ref.add_notification(["not1","not2"])
	ref.add_notification(["not1","not2"])
	ref.add_notification(["not1","not2"])
	ref.add_notification(["not1","not2"])
	ref.add_notification(["not1","not2"])
	ref.add_notification(["not1","not2"])


