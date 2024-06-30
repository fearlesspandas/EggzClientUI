extends Control

class_name ProfilesList

onready var timer:Timer = Timer.new()
onready var label: Label = Label.new()
onready var scroller : ScrollContainer = ScrollContainer.new()
onready var button_container:Control = Control.new()
onready var color_rect : ColorRect = ColorRect.new()
onready var ref_rect : ReferenceRect = ReferenceRect.new()

var button_map = {}

func _ready():
	load_profiles()
	timer.wait_time = 0.5
	timer.connect("timeout",self,"refresh")
	self.add_child(timer)
	timer.start()
	label.text = "PROFILES"
	self.add_child(label)
	scroller.add_child(button_container)
	#button_container.add_child(color_rect)
	ref_rect.editor_only = false
	button_container.add_child(ref_rect)
	self.add_child(scroller)
	
func load_profiles():
	var profiles = ProfileManager.get_all_profiles()
	for i in range(0,profiles.size()):
		var p = profiles[i]
		var button = Button.new()
		button.text = p.id
		button.set_text_align(Button.ALIGN_LEFT)
		button_container.add_child(button)
		button_map[p.id] = button
		
func refresh():
	scroller.set_size(Vector2(self.rect_size.x,self.rect_size.y - label.rect_size.y))
	scroller.set_position(Vector2(0, label.rect_size.y))
	button_container.set_size(scroller.rect_size)
	#scroller.ensure_control_visible(button_container)
	scroller.scroll_vertical_enabled
	ref_rect.set_size(button_container.rect_size)
	ref_rect.border_color = Color.red
	ref_rect.border_width = 5
	color_rect.set_size(button_container.rect_size)
	color_rect.color = Color.red
	var vertical = self.rect_size.y / max(button_map.size() + 1,1)
	var ind = 0
	for i in button_map.keys():
		var button = button_map[i]
		button.text = i
		button.rect_size = Vector2(button_container.rect_size.x,vertical)
		button.set_text_align(Button.ALIGN_LEFT)
		button.set_position(Vector2(0,vertical * ind))
		ind += 1
	
func _process(delta):
	label.set_size(self.rect_size/4)
