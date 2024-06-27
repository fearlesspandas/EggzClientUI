extends Control

class_name NewProfileTab
onready var profiles = find_parent("Profiles")

onready var button:Button = Button.new()
onready var checkbox : CheckBox = CheckBox.new()
onready var textEdit : TextEdit = TextEdit.new()
onready var profiles_list : ProfilesList = ProfilesList.new()

var boundaries:Vector2
var dimensions = Vector2(200,50)
func find_origin_from_center_and_rectsize(center:Vector2,rect_size:Vector2):
	return Vector2(center.x - rect_size.x/2,center.y - rect_size.y/2)
	
func _ready():
	self.name = "+"
	self.set_size(boundaries)
	var center = boundaries/2
	var one_third_offset = boundaries / 3
	self.add_child(textEdit)
	textEdit.set_size(dimensions)
	textEdit.text = "enter new username"
	var text_origin = find_origin_from_center_and_rectsize(center,textEdit.rect_size)
	var text_offset = Vector2(text_origin.x,text_origin.y - one_third_offset.y)
	textEdit.set_global_position(text_offset)
	
	self.add_child(button)
	button.set_size(dimensions)
	button.text = "create new profile"
	var button_origin = find_origin_from_center_and_rectsize(center,button.rect_size)
	var button_offset = Vector2(button_origin.x,button_origin.y)
	button.set_global_position(button_offset)
	
	self.add_child(checkbox)
	checkbox.set_size(dimensions)
	checkbox.text = "start profile as server"
	var checkbox_origin = find_origin_from_center_and_rectsize(center,checkbox.rect_size)
	var checkbox_offset = Vector2(checkbox_origin.x, checkbox_origin.y + one_third_offset.y)
	
	
	self.add_child(profiles_list)
	profiles_list.rect_size = self.rect_size/2.5
	profiles_list.set_global_position(self.rect_size - profiles_list.rect_size)
	pass # Replace with function body.

