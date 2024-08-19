extends Control

class_name DestinationTypeIndicator

onready var destination_direction_type : RichTextLabel = RichTextLabel.new()
onready var destination_gravity_type : RichTextLabel = RichTextLabel.new()

func _ready():
	self.add_child(destination_direction_type)
	self.add_child(destination_gravity_type)
	
func _process(delta):
	self.rect_size = OS.window_size/5
	self.set_position(Vector2(0,OS.window_size.y - self.rect_size.y))
	destination_direction_type.rect_size = Vector2(self.rect_size.x,self.rect_size.y/2)
	destination_gravity_type.rect_size = Vector2(self.rect_size.x,self.rect_size.y/2)
	destination_direction_type.set_position(Vector2(0,0))
	destination_direction_type.set_position(Vector2(0,destination_direction_type.rect_size.y))

func set_destination_mode(mode):
	destination_direction_type.text = str(mode)
