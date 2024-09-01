extends Control

class_name CommandArgument

onready var label:RichTextLabel = RichTextLabel.new()
onready var input_box:TextEdit = TextEdit.new()

var arg_name:String

func size_label():
	label.rect_size = Vector2(self.rect_size.x/2,self.rect_size.y)
	label.set_position(Vector2(0,0))
	
func size_input():
	input_box.rect_size = Vector2(self.rect_size.x/2,self.rect_size.y)
	input_box.set_position(Vector2(label.rect_size.x,0))
		

func _ready():
	assert(arg_name != null and arg_name.length() > 0)
	label.text = arg_name
	label.modulate = Color.red
	self.add_child(label)
	self.add_child(input_box)
	
func _process(delta):
	size_label()
	size_input()
