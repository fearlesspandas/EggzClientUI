extends Control
class_name DestinationListElement
signal delete_destination(uuid)
signal set_active_destination(uuid)

onready var display = get_parent()

var bgRect:ColorRect = ColorRect.new()
var colorRect:ColorRect = ColorRect.new()
var uuid:String
var delete_button:Button = Button.new()
var label_x:RichTextLabel = RichTextLabel.new()
var label_y:RichTextLabel = RichTextLabel.new()
var label_z:RichTextLabel = RichTextLabel.new()
var index:int
var hovering

func _init():
	bgRect.color = Color.lightslategray
	bgRect.mouse_filter = Control.MOUSE_FILTER_PASS
	colorRect.mouse_filter = Control.MOUSE_FILTER_PASS
	delete_button.mouse_filter =Control.MOUSE_FILTER_PASS
	label_x.mouse_filter = Control.MOUSE_FILTER_PASS
	label_y.mouse_filter = Control.MOUSE_FILTER_PASS
	label_z.mouse_filter = Control.MOUSE_FILTER_PASS
	self.add_child(bgRect)
	self.add_child(colorRect)
	self.label_x.scroll_active = false
	self.label_y.scroll_active = false
	self.label_z.scroll_active = false
	self.label_x.visible_characters = 7
	self.label_y.visible_characters = 7
	self.label_z.visible_characters = 7
	delete_button.text = "X"
	delete_button.connect("button_up",self,"delete_dest")
	self.add_child(delete_button)
	self.add_child(label_x)
	self.add_child(label_y)
	self.add_child(label_z)
	

func _ready():
	assert(index != null)
	self.connect("mouse_entered",self,"entered")
	self.connect("mouse_exited",self,"exited")
	pass

func _input(event):
	if event is InputEventMouseButton:
		if event.button_index == 1 and event.is_action_released("left_click") and hovering:
			print_debug("Setting active uuid " , self.uuid)
			emit_signal("set_active_destination",self.uuid)
			
func delete_dest():
	emit_signal("delete_destination",uuid)

func entered():
	hovering = true
	bgRect.color = Color.white
	GlobalSignalsClient.destination_hovered(uuid)

func exited():
	hovering = false
	bgRect.color = Color.lightslategray
	GlobalSignalsClient.destination_unhovered(uuid)
		
func load_dest(destination:Destination):
	self.uuid = destination.uuid
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
	var text_size = Vector2(self.rect_size.x/4,self.rect_size.y - 2*border_size)
	label_x.rect_size = text_size
	label_y.rect_size = text_size
	label_z.rect_size = text_size
	delete_button.rect_size = text_size
	#set positions
	var position = Vector2(0,self.rect_size.y * index)
	self.set_position(position)
	self.colorRect.set_position(Vector2(border_size,border_size))
	self.label_x.set_position(Vector2(border_size,border_size))
	self.label_y.set_position(Vector2(self.label_x.rect_size.x,0) + Vector2(border_size,border_size))
	self.label_z.set_position(Vector2(self.label_x.rect_size.x,0) + Vector2(self.label_y.rect_size.x,0) + Vector2(border_size,border_size))
	self.delete_button.set_position(Vector2(self.label_z.rect_position.x + self.label_z.rect_size.x,0) + Vector2(border_size,border_size))
