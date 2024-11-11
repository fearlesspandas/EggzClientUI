extends Control

class_name MaxSpeedSlider

onready var bg_rect:ColorRect = ColorRect.new()
onready var max_speed_label:RichTextLabel = RichTextLabel.new()
onready var speed_label:RichTextLabel = RichTextLabel.new()

onready var slider:SpeedIndicator = SpeedIndicator.new()

var client_id
var previous_value = 0
var is_subbed = false
var is_active = false

func _ready():
	assert(client_id != null)
	self.add_child(bg_rect)
	self.add_child(speed_label)
	self.add_child(max_speed_label)
	self.add_child(slider)

	bg_rect.color = Color.white
	max_speed_label.modulate = Color.black
	max_speed_label.scroll_active = false
	speed_label.modulate = Color.green
	speed_label.scroll_active = false

	slider.connect("adjust_speed",self,"adjust_speed")
	ClientTerminalGlobalSignals.connect("set_active",self,"set_terminal_active")
	pass


var bg_offset = 3
var bg_offset_vec = Vector2(bg_offset,bg_offset)

func resize():
	if is_active:
		#set sizes
		max_speed_label.rect_size = Vector2(slider.rect_size.x,OS.window_size.y/32)
		speed_label.rect_size = Vector2(slider.rect_size.x,OS.window_size.y/32)
		self.rect_size = Vector2(slider.rect_size.x,slider.rect_size.y + max_speed_label.rect_size.y + speed_label.rect_size.y) + 2*bg_offset_vec
		bg_rect.rect_size = self.rect_size
		#set positions
		self.set_position(OS.window_size - 1.3*self.rect_size ) 
		bg_rect.set_position(Vector2(0,0))
		max_speed_label.set_position(bg_offset_vec)
		speed_label.set_position(Vector2(0,max_speed_label.rect_size.y) + bg_offset_vec)
		slider.set_position(Vector2(0,max_speed_label.rect_size.y + speed_label.rect_size.y) + bg_offset_vec)

func adjust_speed(delta):
	var socket = ServerNetwork.get(client_id)
	if socket != null and delta != 0:
		var stats = {'max_speed_delta':0,'speed_delta':delta}
		socket.adjust_stats(client_id,stats)

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
			slider.max_speed = m
			slider.speed = s
		max_speed_label.text = str(m)
		speed_label.text = str(s)
		resize()
		
func set_active(active:bool):
	is_active = active
	if is_active:
		var socket = ServerNetwork.get(client_id)
		if socket != null:
			socket.get_physical_stats(client_id)

func set_terminal_active(value):
	self.visible = !value
