extends Control

#this menu will display available options such as setting different types of waypoints
#off of click actions.
#Click -> menu -> sub option -> interface scene (likely 3D)

class_name ClickMenu

onready var close_label : ClickOption = ClickOption.new()
onready var waypoint_label : ClickOption = ClickOption.new()
onready var rect:ColorRect = ColorRect.new()

var spawn
var client_id:String
var options = []

var current_position:Vector3
func _ready():
	assert(spawn != null)
	assert(client_id != null)
	close_label.text = "close"
	waypoint_label.text = "waypoint"
	
	options.append(waypoint_label)
	options.append(close_label)
	
	var size = OS.window_size
	self.rect_size = Vector2(size.x / 10, size.y/20 * options.size())
	self.visible = false
	
	rect.color = Color.gray
	rect.rect_size = self.rect_size
	
	self.add_child(rect)
	var ind = 0
	for op in options:
		op.rect_size = Vector2(rect.rect_size.x,rect.rect_size.y/int(options.size()))
		op.set_position(Vector2(0,op.rect_size.y * ind))
		op.connect("option_clicked",self,"spawn_interface")
		rect.add_child(op)
		ind += 1
	
func spawn_interface(label:String):
	match label.to_upper():
		"WAYPOINT":
			assert(current_position != null)
			var waypoint_creator:WaypointCreator = WaypointCreator.new()
			waypoint_creator.client_id = client_id
			waypoint_creator.center = current_position
			spawn.add_child(waypoint_creator)
			self.visible = false
		"CLOSE":
			self.visible = false
		_:
			pass
			
func _input(event):
	if event is InputEventMouseButton and event.is_action_pressed("right_click"):
		self.set_global_position(event.position)
		self.visible = !self.visible
	if event is InputEventKey and event.is_action_pressed("ui_cancel"):
		self.visible = false
		
func handle_clicked(position,button_index):
	#pass position to 'options menu'
	print_debug("Found intersection")
	if button_index == 2:
		current_position = position
		#var waypoint_creator:WaypointCreator = WaypointCreator.new()
		#waypoint_creator.client_id = client_id
		#waypoint_creator.center = position
		#spawn.add_child(waypoint_creator)
		
func player_character_spawned(player:Player):
	assert(player != null)
	player.curserRay.connect("intersection_clicked",self,"handle_clicked")
