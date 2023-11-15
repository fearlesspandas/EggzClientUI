extends Control
class_name PlayerProfileUI
onready var profile:PlayerProfile
onready var file_location:RichTextLabel = find_node("FileLocation")
# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.

func _process(delta):
	if profile != null and file_location != null:
		if profile.file_location != null:
			file_location.text = profile.file_location
		if profile.id != null:
			self

# Called every frame. 'delta' is the elapsed time since the previous frame.
#func _process(delta):
#	pass
