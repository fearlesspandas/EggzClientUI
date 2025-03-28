extends Node
class_name Notifications

onready var ref = load("res://native_lib/Notifications.gdns").new()

onready var parent = get_parent()

func _ready():
	parent.add_child(ref)
	ref.add_notification("test notification",["detail 1","detail 2 "])
	ref.add_notification("test notification",["detail 1","detail 2 "])
	ref.add_notification("test notification",["detail 1","detail 2 "])
	ref.add_notification("test notification",["detail 1","detail 2 "])
	ref.add_notification("test notification",["detail 1","detail 2 "])

	GlobalSignalsClient.connect("killed",self,"display_kill")

func display_kill(id:String):
	ref.add_notification("killed " + id,["reward1","reward2"])


