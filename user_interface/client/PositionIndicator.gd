extends RichTextLabel

class_name PositionIndicator
onready var timer:Timer = Timer.new()

var player


func _process(delta):
	if player != null:
		text = str(player.global_transform.origin)
