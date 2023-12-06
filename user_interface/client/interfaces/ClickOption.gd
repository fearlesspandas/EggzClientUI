extends RichTextLabel

class_name ClickOption
signal option_clicked(label)
var hovered:bool = false
func _ready():
	assert(self.text != null)
	self.connect("mouse_entered",self,"entered")
	self.connect("mouse_exited",self,"exited")
	self.connect("gui_input",self,"gui_input")
	
	
func entered():
	hovered = true
func exited():
	hovered = false

func gui_input(event):
	if event is InputEventMouseButton and event.is_action_pressed("left_click"):
		print("label clicked")
		emit_signal("option_clicked",self.text)
func _process(delta):
	if hovered:
		self.modulate = Color.green
	else:
		self.modulate = Color.red


	
