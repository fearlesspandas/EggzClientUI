extends Control
class_name Command
signal button_clicked(args)

onready var bg_rect:ColorRect = ColorRect.new()
onready var border_rect:ColorRect = ColorRect.new()
onready var request_button:Button = Button.new()

var border_offset = 5
var border_offset_vec = Vector2(border_offset,border_offset)
var command_name:String
var arguments = []

func add_args(args):
	for arg in args:
		var ui_arg : CommandArgument = CommandArgument.new()
		ui_arg.arg_name = arg
		arguments.push_back(ui_arg)
		
func size_args():
	for i in range(0,arguments.size()):
		var arg = arguments[i]
		if arg is CommandArgument:
			arg.rect_size = Vector2(bg_rect.rect_size.x/4,bg_rect.rect_size.y)
			arg.set_position(Vector2(request_button.rect_position.x + request_button.rect_size.x + arg.rect_size.x * i,bg_rect.rect_position.y))

func size_button():
	request_button.rect_size = Vector2(bg_rect.rect_size.x/4,bg_rect.rect_size.y)
	request_button.set_position(Vector2(bg_rect.rect_position.x,bg_rect.rect_position.y))

func size():
	self.rect_size = Vector2(OS.window_size.x,OS.window_size.y/10)
	border_rect.rect_size = self.rect_size
	bg_rect.rect_size = border_rect.rect_size - 2*border_offset_vec
	bg_rect.set_position(border_rect.rect_position + border_offset_vec)
	size_button()
	size_args()
	
func _ready():
	assert(command_name != null and command_name.length() > 0)
	self.add_child(border_rect)
	border_rect.color = Color.white
	self.add_child(bg_rect)
	bg_rect.color = Color.black
	request_button.connect("button_up",self,"button_clicked")
	self.add_child(request_button)
	request_button.text = "send request " + command_name
	for arg in arguments:
		self.add_child(arg)
		
func button_clicked():
	var argmap = {}
	for arg in arguments:
		if arg is CommandArgument:
			argmap[arg.arg_name] = arg.input_box.text
	emit_signal("button_clicked",argmap)
	
func _process(delta):
	size()
