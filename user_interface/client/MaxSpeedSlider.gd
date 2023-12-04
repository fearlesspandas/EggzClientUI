extends VSlider

class_name MaxSpeedSlider

var client_id
var previous_value = 0
var is_subbed = false
func _ready():
	#self.connect("value_changed",self,"slider_update")
	#subscribe to max_speed
	pass
		
func slider_update(value:float):
	print("slider update", value)
	var socket = ServerNetwork.get(client_id)
	var stats = {'speed_delta':value - previous_value}
	if socket != null:
		socket.adjust_stats(client_id,stats)
	previous_value = value
	
func _input(event):
	var delta = 0
	if event is InputEventKey and event.is_action_pressed("up_arrow",true):
		delta +=1
	if event is InputEventKey and event.is_action_pressed("down_arrow",true):
		delta -= 1
	var socket = ServerNetwork.get(client_id)
	if socket != null and delta != 0:
		var stats = {'speed_delta':delta}
		socket.adjust_stats(client_id,stats)
		
		
func _process(delta):
	var m = DataCache.cached(client_id,'max_speed')
	#print("max speed slider, cache " , m)
	if m != null:
		self.tick_count = ceil(2 * m)
		self.value = m
	if ! is_subbed:
		var socket = ServerNetwork.get(client_id)
		if socket!= null:
			#print("max speed slider sending phys stats subscribe",client_id)
			var query = PayloadMapper.get_physical_stats(client_id)
			#print("max speed slider sending phys stats subscribe",client_id,query)
			#socket.subscribe_general(query)
			socket.get_physical_stats(client_id)
			#is_subbed = true
	
