extends Timer

class_name EntityScannerTimer
var queue_entities = []
var client_id
var is_active:bool = false
func _ready():
	self.connect("timeout",self,"poll_for_entities")
	pass # Replace with function body.

func poll_for_entities():
	if is_active:
		ServerNetwork.get(client_id).getAllGlobs()
		
func set_active(active:bool):
	is_active = active
