extends Control

class_name DestinationTypeIndicator

onready var destination_direction_type : RichTextLabel = RichTextLabel.new()
#onready var destination_gravity_type : RichTextLabel = RichTextLabel.new()
onready var destinations_active_bg:ColorRect = ColorRect.new()
onready var gravity_active_bg:ColorRect = ColorRect.new()
onready var main_bg:ColorRect = ColorRect.new()

var gravity_active:bool
var destinations_active:bool

var bg_offset = 5
var bg_offset_vec = Vector2(bg_offset,bg_offset)

func _ready():
	self.add_child(destinations_active_bg)
	self.add_child(gravity_active_bg)
	self.add_child(main_bg)
	self.add_child(destination_direction_type)
	main_bg.color = Color.black
	self.mouse_filter = Control.MOUSE_FILTER_IGNORE
	
func _process(delta):
	self.rect_size = Vector2(OS.get_window_safe_area().size.x/5,OS.get_window_safe_area().size.y/10)
	destinations_active_bg.rect_size = self.rect_size
	gravity_active_bg.rect_size = destinations_active_bg.rect_size - 2*bg_offset_vec
	main_bg.rect_size = gravity_active_bg.rect_size - 2*bg_offset_vec
	destination_direction_type.rect_size = Vector2(self.rect_size.x/3,self.rect_size.y/3)
	self.set_position(Vector2(0,OS.get_window_safe_area().size.y - self.rect_size.y))
	destinations_active_bg.set_position(Vector2(0,0))
	gravity_active_bg.set_position(destinations_active_bg.rect_position + bg_offset_vec)
	main_bg.set_position(gravity_active_bg.rect_position + bg_offset_vec)
	destination_direction_type.set_position(
		Vector2(
			self.rect_size.x/2 - destination_direction_type.rect_size.x/2,
			self.rect_size.y/2 - destination_direction_type.rect_size.y/2
			)
		)
	
func set_destination_mode(mode):
	destination_direction_type.text = str(mode)

func set_gravity_active(gravity_on:bool):
	if gravity_on:
		gravity_active_bg.color = Color.purple
	else:
		gravity_active_bg.color = Color.white

func set_destinations_active(is_active:bool):
	if is_active:
		destinations_active_bg.color = Color.greenyellow
	else:
		destinations_active_bg.color = Color.gray
