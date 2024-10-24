
extends RichTextLabel

class_name PhysicsDataStats

onready var timer:Timer = Timer.new()
onready var message_controller:MessageController = MessageController.new()
onready var parent = get_parent()
var is_active:bool = false
var client_id
var base_label = "Location Latency:"

var last:int = 0
func _ready():
	self.text = "NOTHING"
	self.add_child(timer)
	self.add_child(message_controller)
	GlobalSignalsClient.connect("location_received",self,"location_received")
	
	
func location_received(id):
	if id == client_id:
		var curr = Time.get_ticks_usec()
		self.text = base_label + str(curr - last)
		last = curr
		
	
			

