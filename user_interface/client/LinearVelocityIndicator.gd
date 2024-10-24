extends RichTextLabel

class_name LinearVelocityIndicator

onready var timer:Timer = Timer.new()
onready var message_controller:MessageController = MessageController.new()
onready var parent = get_parent()
var is_active:bool = false
var client_id
var base_label = "LV:"

func _ready():
	self.text = "NOTHING"
	timer.wait_time = 0.25
	timer.connect("timeout",self,"timeout_polling")
	self.add_child(timer)
	self.add_child(message_controller)
	timer.start()
	
func timeout_polling():
	if is_active:
		var socket = ServerNetwork.get(client_id)
		if socket != null:
			socket.lazy_lv(client_id)
		self.text = base_label + str(DataCache.cached(client_id,'lv'))
			

