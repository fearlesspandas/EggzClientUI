extends ColorRect

class_name ConnectionIndicator

var client_id
func _ready():
	pass # Replace with function body.

func _process(delta):
	if ServerNetwork.get(client_id) == null:
		self.color = Color.red
	elif ServerNetwork.get(client_id).connected:
		self.color = Color.green
	else:
		self.color = Color.red
	pass
