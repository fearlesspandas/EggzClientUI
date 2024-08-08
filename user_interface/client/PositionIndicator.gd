extends RichTextLabel

class_name PositionIndicator
onready var timer:Timer = Timer.new()

var player


func _process(delta):
	if player != null:
		text = str(player.body.global_transform.origin)

func player_character_spawned(player:Player):
	assert(player != null)
	self.player = player
