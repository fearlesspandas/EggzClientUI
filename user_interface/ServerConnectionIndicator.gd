extends ColorRect


# Declare member variables here. Examples:
# var a = 2
# var b = "text"


# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.

func _process(delta):
	if ServerNetwork.socket != null and ServerNetwork.socket.connected:
		self.color = Color.green
	else:
		self.color = Color.red
# Called every frame. 'delta' is the elapsed time since the previous frame.
#func _process(delta):
#	pass
