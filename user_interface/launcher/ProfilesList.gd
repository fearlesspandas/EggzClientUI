extends Control

class_name ProfilesList

onready var timer:Timer = Timer.new()
onready var label: Label = Label.new()

func _ready():
	refresh()
	timer.wait_time = 1
	timer.connect("timeout",self,"refresh")
	self.add_child(timer)
	timer.start()
	label.text = "PROFILES"
	
func refresh():
	var profiles = ProfileManager.get_all_profiles()
	var vertical = self.rect_size.y / max(profiles.size(),1)
	print_debug("refreshing profiles",profiles)
	for i in range(0,profiles.size()):
		var p = profiles[i]
		var button = Button.new()
		button.text = p.id
		button.rect_size = Vector2(self.rect_size.x,vertical)
		button.set_text_align(Button.ALIGN_LEFT)
		button.set_global_position(Vector2(0,vertical * i))
		self.add_child(button)
	
func _process(delta):
	label.set_size(self.rect_size)
