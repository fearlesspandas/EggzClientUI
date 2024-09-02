extends RichTextLabel

class_name PositionIndicator
onready var timer:Timer = Timer.new()

var player


func _process(delta):
	if player != null:
		text = str(player.body.global_transform.origin)
	self.rect_size = Vector2(OS.window_size.x/4,100)
	self.set_position(OS.window_size - self.rect_size)
	
func player_character_spawned(player:Player):
	assert(player != null)
	self.player = player
