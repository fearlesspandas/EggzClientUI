extends Control

class_name Console

onready var frame:Control = Control.new()
onready var input:TextEdit = TextEdit.new()
onready var output:RichTextLabel = RichTextLabel.new()
onready var message_controller = MessageController.new()
onready var parent:Control = get_parent()

var client_id:String
#func _input(event):
#	if event is InputEventKey and event.is_pressed("Enter"):
#		var payload = input.text

func _ready():
	#size node frames
	var frame_size = Vector2(500,250)
	frame.rect_size = frame_size
	input.rect_size = frame.rect_size/2
	output.rect_size = frame.rect_size/2
	#add elements to scene
	self.add_child(frame)
	frame.add_child(input)
	frame.add_child(output)
	#position elements
	frame.set_position(Vector2(0,parent.rect_size.y - frame_size.y))
	output.set_position(Vector2(0,0))
	input.set_position(Vector2(0,frame.rect_size.y/2))
	#connect signals
	self.add_child(message_controller)

func _on_data():
	#pass
	assert(false)
	var cmd = ServerNetwork.get(client_id).get_packet(true)
	message_controller.add_to_queue(cmd.left(10))
	
func _handle_message(msg,delta):
	output.text = msg.left(10)

func _process(delta):
	#output.text = input.text
	pass
