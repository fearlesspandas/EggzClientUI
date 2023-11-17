extends Timer

var queue_entities = []
var client_id
func _ready():
	self.wait_time = 1
	pass # Replace with function body.

func poll_for_entities():
	ServerNetwork.get(client_id).getAllGlobs()
