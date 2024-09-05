extends Control
class_name DestinationWindow
onready var destination_display:DestinationDisplay = DestinationDisplay.new()
onready var destination_window_display_bar:ColorRect = ColorRect.new()
onready var destinations_window_label:RichTextLabel = RichTextLabel.new()

var is_open:bool = false

func _ready():
	self.add_child(destination_window_display_bar)
	destinations_window_label.text = "Destinations"
	destination_window_display_bar.add_child(destinations_window_label)

func size_display_bar():
	#set sizes//////////////////////
	var expanded_size = Vector2(0,0)
	if is_open:
		expanded_size = Vector2(0,destination_display.rect_size.y)

	self.rect_size = Vector2(OS.window_size.y/16,OS.window_size.x/4) + expanded_size
	destination_window_display_bar.rect_size = self.rect_size 
	destinations_window_label.rect_size = destination_window_display_bar.rect_size
	#set positions//////////////////
	self.set_position(Vector2(
		OS.window_size.x - self.rect_size.x,
		0
	))
	destination_window_display_bar.set_position(Vector2(0,0))
	destinations_window_label.set_position(Vector2(0,0))
	destination_display.set_position(Vector2(0,destination_window_display_bar.rect_size.y))
	
func add_destination_display():
	self.add_child(destination_display)

func remove_destination_display():
	self.remove_child(destination_display)
	destination_display.call_deferred('free')

func toggle_is_open():
	is_open = !is_open

func _input(event):
	if event is InputEventMouseButton:
		if event.button_index == 1 and event.is_action_released("left_click") :
			toggle_is_open()
			if self.is_open:
				add_destination_display()
				emit_signal("load_destinations")
			else:
				remove_destination_display()

func _process(delta):
	size_display_bar()
