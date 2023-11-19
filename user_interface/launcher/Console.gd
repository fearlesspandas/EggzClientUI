extends Node

class_name Console

onready var frame:Control = Control.new()
onready var input:TextEdit = TextEdit.new()
onready var output:RichTextLabel = RichTextLabel.new()
onready var message_controller = MessageController.new()

var type = "CLIENT"

func _ready():
	
	#message_controller.isClient
	#size node frames
	frame.rect_size = Vector2(500,500)
	input.rect_size = frame.rect_size/2
	output.rect_size = frame.rect_size/2
	#add elements to scene
	self.add_child(frame)
	frame.add_child(input)
	frame.add_child(output)
	#position elements
	output.set_position(Vector2(0,0))
	input.set_position(Vector2(0,frame.rect_size.y/2))
	
	#connect signals
	
	self.add_child(message_controller)
	pass # Replace with function body.
