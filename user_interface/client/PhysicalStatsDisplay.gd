extends Control
class_name PhysicalStatsDisplay

onready var bg_rect:ColorRect = ColorRect.new()
onready var lv_indicator: LinearVelocityIndicator = LinearVelocityIndicator.new()
onready var position_indicator: PositionIndicator = PositionIndicator.new()

var is_active:bool = false

func _ready():
	self.add_child(lv_indicator)
	self.add_child(position_indicator)
	self.add_child(bg_rect)
	bg_rect.color = Color.black
	bg_rect.color.a = 0.7
	self.visible = false


func size_and_position():
	#set size
	self.rect_size = OS.window_size/4
	bg_rect.rect_size = self.rect_size
	lv_indicator.rect_size = Vector2(self.rect_size.x,30)
	position_indicator.rect_size = Vector2(self.rect_size.x,30)

	#set position
	self.set_position(Vector2((OS.window_size.x - self.rect_size.x)/2,0))
	bg_rect.set_position(Vector2(0,0))
	lv_indicator.set_position(Vector2(0,0))
	position_indicator.set_position(Vector2(0,lv_indicator.rect_size.y))

func set_active(value:bool):
	self.is_active = value

func _input(event):
	if event is InputEventKey and is_active:
		if event.is_action_released("toggle_physical_stats"):
			self.visible = !self.visible

func _process(delta):
	if is_active and self.visible:
		lv_indicator.is_active = true
		position_indicator.is_active = true
		size_and_position()
	else:
		lv_indicator.is_active = false
		position_indicator.is_active = false
