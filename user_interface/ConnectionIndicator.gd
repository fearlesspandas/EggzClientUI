extends ColorRect

class_name ConnectionIndicator

var socket:ClientWebSocket
func _ready():
	pass # Replace with function body.

func _process(delta):
	if socket == null:
		self.color = Color.red
	elif socket.connected:
		self.color = Color.green
	else:
		self.color = Color.red
	pass
