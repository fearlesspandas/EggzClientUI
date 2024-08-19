extends Control
class_name DestinationListElement

onready var display = get_parent()

var bgRect:ColorRect = ColorRect.new()
var colorRect:ColorRect = ColorRect.new()
var label_x:RichTextLabel = RichTextLabel.new()
var label_y:RichTextLabel = RichTextLabel.new()
var label_z:RichTextLabel = RichTextLabel.new()
var index:int
func _init():
	bgRect.color = Color.lightslategray
	self.add_child(bgRect)
	self.add_child(colorRect)
	self.label_x.scroll_active = false
	self.label_y.scroll_active = false
	self.label_z.scroll_active = false
	self.label_x.visible_characters = 7
	self.add_child(label_x)
	self.add_child(label_y)
	self.add_child(label_z)
	

func _ready():
	assert(index != null)
	pass
	
func load_dest(destination:Destination):
	self.colorRect.color = destination.material.albedo_color
	self.label_x.text = str(round(destination.location.x))
	self.label_y.text = str(round(destination.location.y))
	self.label_z.text = str(round(destination.location.z))
	
func _process(delta):
	var size_y = clamp(display.rect_size.y/display.all_destinations.size(),10,50)
	var size = Vector2(display.rect_size.x,size_y)
	self.rect_size = size
	self.bgRect.rect_size = self.rect_size
	var border_size = 2
	self.colorRect.rect_size = self.rect_size - 2 * Vector2(border_size,border_size)
	var text_size = Vector2(self.rect_size.x/3,self.rect_size.y - 2*border_size)
	label_x.rect_size = text_size
	label_y.rect_size = text_size
	label_z.rect_size = text_size
	#set positions
	var position = Vector2(0,self.rect_size.y * index)
	self.set_position(position)
	self.colorRect.set_position(Vector2(border_size,border_size))
	self.label_x.set_position(Vector2(border_size,border_size))
	self.label_y.set_position(Vector2(self.label_x.rect_size.x,0) + Vector2(border_size,border_size))
	self.label_z.set_position(Vector2(self.label_x.rect_size.x,0) + Vector2(self.label_y.rect_size.x,0) + Vector2(border_size,border_size))
