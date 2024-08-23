extends Control

class_name DestinationDisplay
signal delete_destination(uuid)


var all_destinations = {}
var index:int
var position_indicator:CurrentDestinationIndicator = CurrentDestinationIndicator.new()

func _ready():
	self.rect_size = OS.window_size/4
	#self.position_indicator.rect_size = self.rect_size/16
	self.add_child(position_indicator)

func reposition_index_indicator():
	if index!= null and all_destinations.size() > 0:
		position_indicator.visible = true
		var element_pos = all_destinations.values()[index]
		var size = Vector2(self.rect_size.x/16,element_pos.rect_size.y)
		position_indicator.rect_size = size
		var position = Vector2(
			element_pos.get_position().x - size.x,
			element_pos.get_position().y
		)
		position_indicator.set_position(position)
	else:
		#assert(false)
		position_indicator.visible = false
		
func add_destination(dest:Destination):
	var element = DestinationListElement.new()
	element.load_dest(dest)
	all_destinations[dest.uuid] = element
	element.index = all_destinations.size()-1
	self.add_child(element)
	element.connect("delete_destination",self,"delete_destination")
	#reposition_index_indicator()

func delete_destination(uuid):
	emit_signal("delete_destination",uuid)
	
func add_destination_if_not_present(dest:Destination):
	if all_destinations.has(dest.uuid):
		pass
	else:
		add_destination(dest)
	#reposition_index_indicator()
	
func erase_destinations():
	for dest in all_destinations:
		self.remove_child(dest)
		if dest != null:
			dest.call_deferred("free")
			dest = null
	all_destinations = []
	#position_indicator.visible = false

func erase_destinations_if_not_present(incoming_destinations:Array):
	var incoming_uuid = {}
	for dest in incoming_destinations:
		if dest is Destination:
			incoming_uuid[dest.uuid] = dest
	for dest in all_destinations.values():
		if incoming_uuid.has(dest.uuid):
			pass
		else:
			self.remove_child(dest)
			if dest != null:
				dest.call_deferred("free")
				dest = null
			all_destinations.erase(dest.uuid)
	#position_indicator.visible = false
	
func refresh_destinations(destinations):
	#reposition_index_indicator()
	#erase_destinations()
	erase_destinations_if_not_present(destinations)
	for d in destinations:
		add_destination_if_not_present(d)
	reposition_index_indicator()

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
