extends Control

class_name DestinationDisplay



var all_destinations = []
var index:int
var position_indicator:CurrentDestinationIndicator = CurrentDestinationIndicator.new()

func _ready():
	self.rect_size = OS.window_size/4
	self.position_indicator.rect_size = self.rect_size/16
	self.add_child(position_indicator)
	
func reposition_index_indicator():
	if all_destinations.size() > 0 and index!= null and index < all_destinations.size():
		position_indicator.visible = true
		var element_pos = all_destinations[index]
		var position = Vector2(
			element_pos.get_position().x - 50,
			element_pos.get_position().y
		)
		position_indicator.set_position(position)
	else:
		position_indicator.visible = false
		
func add_destination(dest:Destination):
	var element = DestinationListElement.new()
	element.load_dest(dest)
	all_destinations.push_back(element)
	element.index = all_destinations.size()-1
	self.add_child(element)
	#var position = Vector2(0,element.rect_size.y * element.index)
	#element.set_position(position)
	
func erase_destinations():
	for dest in all_destinations:
		self.remove_child(dest)
		if dest != null:
			dest.call_deferred("free")
			dest = null
	all_destinations = []
	#position_indicator.visible = false
	
func refresh_destinations(destinations):
	erase_destinations()
	for d in destinations:
		add_destination(d)

func set_index(ind:int):
	index = ind
	reposition_index_indicator()
	
func _process(delta):
	#var index = DataCache.cached("","index")
	#if index != null:
	#	self.index = index
	#reposition_index_indicator()
	self.rect_size = OS.window_size/4
	self.set_position(Vector2(
		OS.window_size.x - self.rect_size.x,
		0
	))
