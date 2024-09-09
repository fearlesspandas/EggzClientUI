extends Control
class_name SpeedIndicator

signal adjust_speed(delta)

onready var bg_rect:ColorRect = ColorRect.new()
onready var speed_rect:ColorRect = ColorRect.new()
onready var hover_rect:ColorRect = ColorRect.new()
var max_speed:float = 0
var speed:float = 0

var hovering:bool = false
func _ready():
	self.add_child(bg_rect)
	self.add_child(speed_rect)
	self.add_child(hover_rect)
	bg_rect.color = Color.black
	speed_rect.color = Color.red
	hover_rect.color = Color.yellow
	hover_rect.color.a = 0.5
	bg_rect.mouse_filter = Control.MOUSE_FILTER_PASS
	speed_rect.mouse_filter = Control.MOUSE_FILTER_PASS
	hover_rect.mouse_filter = Control.MOUSE_FILTER_PASS
	self.connect("mouse_entered",self,"entered")
	self.connect("mouse_exited",self,"exited")


func entered():
	hovering = true

func exited():
	hovering = false

func size_and_position():
	#Set sizes
	self.rect_size = Vector2(OS.window_size.x/32,OS.window_size.y/4)
	bg_rect.rect_size = self.rect_size
	speed_rect.rect_size = Vector2(bg_rect.rect_size.x,get_speed_ratio() * bg_rect.rect_size.y)
	#Set positions
	bg_rect.set_position(Vector2(0,0))
	speed_rect.set_position(Vector2(0,bg_rect.rect_size.y - speed_rect.rect_size.y))
	#set visible
	if hovering:
		hover_rect.visible = true
	else:
		hover_rect.visible = false


func get_speed_ratio() -> float:
	if max_speed > 0 :
		return speed/max_speed
	else:
		return 0.0

func get_ratio(numerator:float,denominator:float) -> float:
	if denominator > 0:
		return numerator/denominator
	else:
		return 0.0

func _input(event):
	if event is InputEventMouseMotion and hovering:
		hover_rect.rect_size = Vector2(
			speed_rect.rect_size.x,
			min(bg_rect.rect_size.y - (event.position.y - bg_rect.get_global_position().y),bg_rect.rect_size.y)
		)
		hover_rect.set_position(Vector2(0,bg_rect.rect_size.y - hover_rect.rect_size.y)) 
	if event is InputEventMouseButton and hovering and event.is_action_released("left_click"):
		var new_ratio = get_ratio(hover_rect.rect_size.y,bg_rect.rect_size.y)	
		var old_ratio = get_ratio(speed_rect.rect_size.y,bg_rect.rect_size.y)
		emit_signal("adjust_speed",(new_ratio - old_ratio) * max_speed)
			

func _process(delta):
	size_and_position()
