extends Control

class_name SettingsMenu
onready var camera_render_distance_input:TextEdit = TextEdit.new()

func _ready():
	self.rect_size = OS.window_size/2
	self.set_position(OS.window_size/2 - self.rect_size/2)
	camera_render_distance_input.rect_size = self.rect_size/2
	self.add_child(camera_render_distance_input)
	self.visible = false
	
func _process(delta):
	if Input.is_action_just_pressed("ui_cancel"):
		self.visible = not self.visible
	
