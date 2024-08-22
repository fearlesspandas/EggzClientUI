extends VSlider

class_name MaxSpeedSlider

var client_id
var previous_value = 0
var is_subbed = false
var is_active = false
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
	if is_active:
		var delta = 0
		if event is InputEventKey and event.is_action_pressed("up_arrow",true):
			delta +=1
		if event is InputEventKey and event.is_action_pressed("down_arrow",true):
			delta -= 1
		var socket = ServerNetwork.get(client_id)
		if socket != null and delta != 0:
			var stats = {'max_speed_delta':delta,'speed_delta':delta}
			socket.adjust_stats(client_id,stats)
		
		
func _process(delta):
	if is_active:
		var m = DataCache.cached(client_id,'max_speed')
		var s = DataCache.cached(client_id,'speed')
		#print("max speed slider, cache " , m)
		if m != null:
			self.tick_count = ceil(2 * m)
			self.value = s
	
func set_active(active:bool):
	is_active = active
	if is_active:
		ServerNetwork.get(client_id).get_physical_stats(client_id)
