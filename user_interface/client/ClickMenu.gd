extends Control

class_name ClickMenu

onready var close_label : RichTextLabel = RichTextLabel.new()
onready var rect:ColorRect = ColorRect.new()
var options = []

func _ready():
	self.rect_size = OS.window_size / 20
	self.visible = false
	close_label.text = "close"
	rect.color = Color.gray
	rect.rect_size = self.rect_size
	
	self.add_child(rect)
	rect.add_child(close_label)	
	options.append(close_label)
	close_label.rect_size = Vector2(rect.rect_size.x,rect.rect_size.y/int(options.size()))
	
	
func _input(event):
	if event is InputEventMouseButton and event.is_action_pressed("right_click"):
		self.set_global_position(event.position)
		self.visible = !self.visible
	
