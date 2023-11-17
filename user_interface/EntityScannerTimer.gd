extends Timer

class_name EntityScannerTimer
var queue_entities = []
var client_id
var isClient:bool = true
func _ready():
	self.connect("timeout",self,"poll_for_entities")
	pass # Replace with function body.

func poll_for_entities():
	ServerNetwork.get(client_id,isClient).getAllGlobs()
