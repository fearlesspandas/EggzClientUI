extends Timer

class_name DestinationScannerTimer
var queue_entities = []
var client_id
var is_active:bool = false
func _ready():
	self.connect("timeout",self,"poll_for_entities")
	#ServerNetwork.get(client_id).get_all_destinations(client_id)
	pass

func poll_for_entities():
	if is_active:
		ServerNetwork.get(client_id).get_all_destinations(client_id)
		
func set_active(active:bool):
	is_active = active
