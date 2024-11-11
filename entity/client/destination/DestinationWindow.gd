extends Control
class_name DestinationWindow

signal load_destinations

onready var destination_display:DestinationDisplay = DestinationDisplay.new()
onready var destination_window_display_bar:ColorRect = ColorRect.new()
onready var destinations_window_label:RichTextLabel = RichTextLabel.new()
onready var bg_rect:ColorRect = ColorRect.new()

var is_open:bool = false
var hovering:bool = false

var bg_offset = 3
var bg_offset_vec = Vector2(bg_offset,bg_offset)

func _ready():
	self.add_child(bg_rect)
	self.add_child(destination_window_display_bar)
	destination_window_display_bar.add_child(destinations_window_label)
	self.add_child(destination_display)
	self.connect("mouse_entered",self,"entered")
	self.connect("mouse_exited",self,"exited")
	bg_rect.color = Color.blue
	bg_rect.color.a = 0.5
	destinations_window_label.text = "DESTINATIONS"
	destinations_window_label.visible_characters = 12
	destinations_window_label.modulate = Color.white
	destination_window_display_bar.color = Color.black
	destination_window_display_bar.mouse_filter = Control.MOUSE_FILTER_PASS
	destinations_window_label.mouse_filter = Control.MOUSE_FILTER_PASS
	destination_display.visible = is_open
	ClientTerminalGlobalSignals.connect("set_active",self,"set_terminal_active")

func entered():
	hovering = true
	bg_rect.color = Color.white
	bg_rect.color.a = 0.5

func exited():
	hovering = false
	bg_rect.color = Color.blue
	bg_rect.color.a = 0.5

func size_display_bar():
	#set sizes//////////////////////
	var expanded_size = Vector2(0,0)
	var base_size = Vector2(OS.window_size.x/4,OS.window_size.y/32)
	if is_open:
		expanded_size = Vector2(0,destination_display.rect_size.y)
	self.rect_size = base_size  + expanded_size + 2 * bg_offset_vec
	bg_rect.rect_size = self.rect_size
	destination_window_display_bar.rect_size = base_size
	destinations_window_label.rect_size = Vector2(destination_window_display_bar.rect_size.x/2,destination_window_display_bar.rect_size.y)
	#set positions//////////////////
	self.set_position(Vector2(
		OS.window_size.x - (self.rect_size.x + bg_offset),
		0
	))
	destination_window_display_bar.set_position(Vector2(0,0) + bg_offset_vec)
	destinations_window_label.set_position(Vector2(destination_window_display_bar.rect_size.x/2 - destinations_window_label.rect_size.x/2,0)  + bg_offset_vec)
	destination_display.set_position(Vector2(0,destination_window_display_bar.rect_size.y) + bg_offset_vec)
	
func add_destination_display():
	destination_display.visible = true
	emit_signal("load_destinations")

func remove_destination_display():
	destination_display.visible = false

func toggle_is_open():
	is_open = !is_open

func _input(event):
	if event is InputEventMouseButton:
		if event.button_index == 1 and event.is_action_released("left_click") and hovering:
			toggle_is_open()
			if self.is_open:
				add_destination_display()
			else:
				remove_destination_display()

func _process(delta):
	size_display_bar()

func set_terminal_active(value):
	self.visible = !value
