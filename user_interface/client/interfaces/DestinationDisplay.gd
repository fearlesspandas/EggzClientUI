extends Control

class_name DestinationDisplay

onready var colorRect:ColorRect = ColorRect.new()

var all_destinations = []

func _ready():
	colorRect.color = Color.aqua
	self.rect_size = OS.window_size/4
	colorRect.rect_size = self.rect_size
	#self.add_child(colorRect)
	
func add_destination(dest:Destination):
	var element = DestinationListElement.new()
	element.load_dest(dest)
	all_destinations.push_back(element)
	element.index = all_destinations.size()-1
	self.add_child(element)
	var position = Vector2(0,element.rect_size.y * element.index)
	element.set_position(position)
	
func erase_destinations():
	for dest in all_destinations:
		self.remove_child(dest)
		if dest != null:
			dest.call_deferred("free")
			dest = null
	all_destinations = []
	
func refresh_destinations(destinations):
	erase_destinations()
	for d in destinations:
		add_destination(d)

func _process(delta):
	self.rect_size = OS.window_size/4
	self.set_position(Vector2(
		OS.window_size.x - self.rect_size.x,
		0
	))
