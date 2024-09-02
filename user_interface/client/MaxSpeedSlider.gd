extends Control

class_name MaxSpeedSlider

onready var max_speed_label:RichTextLabel = RichTextLabel.new()
onready var speed_label:RichTextLabel = RichTextLabel.new()

onready var slider:VSlider = VSlider.new()

onready var resize_timer:Timer = Timer.new()
var client_id
var previous_value = 0
var is_subbed = false
var is_active = false

func _ready():
	assert(client_id != null)
	self.add_child(speed_label)
	self.add_child(max_speed_label)
	self.add_child(slider)
	self.add_child(resize_timer)
	resize_timer.wait_time = 3
	resize_timer.connect("timeout",self,"resize")
	resize_timer.start()
	pass


func resize():
	if is_active:
		self.rect_size = OS.window_size/4
		slider.rect_size = Vector2(self.rect_size.x/3,self.rect_size.y * 0.5)
		max_speed_label.rect_size = Vector2(self.rect_size.x/5,20)
		speed_label.rect_size = Vector2(self.rect_size.x/5,20)
		self.set_position(OS.window_size - self.rect_size)
		slider.set_position(Vector2(self.rect_size.x/2,self.rect_size.y- 2 * slider.rect_size.y))
		speed_label.set_position(slider.rect_position - speed_label.rect_size)
		max_speed_label.set_position(Vector2(self.rect_size.x - speed_label.rect_size.x,0))
		
func _input(event):
	if is_active:
		var delta = 0
		if event is InputEventKey and event.is_action_pressed("up_arrow",true):
			delta +=1
		if event is InputEventKey and event.is_action_pressed("down_arrow",true):
			delta -= 1
		var socket = ServerNetwork.get(client_id)
		if socket != null and delta != 0:
			var stats = {'max_speed_delta':0,'speed_delta':delta}
			socket.adjust_stats(client_id,stats)
		
		
func _process(delta):
	if is_active:
		var m = DataCache.cached(client_id,'max_speed')
		var s = DataCache.cached(client_id,'speed')
		
		#print("max speed slider, cache " , m)
		if m != null:
			slider.tick_count = ceil(m)
			slider.value = s
		max_speed_label.text = str(m)
		speed_label.text = str(s)
		
func set_active(active:bool):
	is_active = active
	if is_active:
		var socket = ServerNetwork.get(client_id)
		if socket != null:
			socket.get_physical_stats(client_id)
