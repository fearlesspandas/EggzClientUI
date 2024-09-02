extends RichTextLabel

class_name LinearVelocityIndicator

onready var timer:Timer = Timer.new()
onready var message_controller:MessageController = MessageController.new()
onready var parent = get_parent()
var client_id

func _ready():
	self.text = "NOTHING"
	timer.wait_time = 0.25
	timer.connect("timeout",self,"timeout_polling")
	self.add_child(timer)
	self.add_child(message_controller)
	timer.start()
	
func timeout_polling():
	var socket = ServerNetwork.get(client_id)
	if socket != null:
		socket.lazy_lv(client_id)
	self.text = str(DataCache.cached(client_id,'lv'))
	
func _handle_message(msg,delta):
	match msg:
		{'LV':{'id':var id,'lv': [var x , var y , var z]}}:
			self.text = str([x,y,z])
			

