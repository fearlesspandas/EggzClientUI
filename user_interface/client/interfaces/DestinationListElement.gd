extends ColorRect
class_name DestinationListElement

onready var display = get_parent()

var label:RichTextLabel = RichTextLabel.new()
var index:int
func _init():
	self.add_child(label)

func _ready():
	assert(index != null)
	pass
	
func load_dest(destination:Destination):
	self.color = destination.material.albedo_color
	self.label.text = str(destination.location)
	
func _process(delta):
	var size_y = clamp(display.rect_size.y/display.all_destinations.size(),10,50)
	var size = Vector2(display.rect_size.x,size_y)
	self.rect_size = size
	label.rect_size = self.rect_size / 2
	var position = Vector2(0,self.rect_size.y * index)
	self.set_position(position)
	#label.set_position(self.rect_size / 2 - label.rect_size/2)
