extends Control

class_name DestinationDisplay
signal delete_destination(uuid)
signal set_active_destination(uuid)
onready var reposition_index_timer:Timer = Timer.new()

var all_destinations = {}
var index:int
var position_indicator:CurrentDestinationIndicator = CurrentDestinationIndicator.new()

func _ready():
	self.rect_size = OS.window_size/4
	#self.position_indicator.rect_size = self.rect_size/16
	self.add_child(position_indicator)
	#reposition_index_timer.wait_time = 1
	#reposition_index_timer.connect("timeout",self,"reposition_index_indicator")
	#self.add_child(reposition_index_timer)
	#sreposition_index_timer.start()

func reposition_index_by_id(uuid):
	for dest in all_destinations.values():
		if dest is Destination and dest.uuid == uuid:
			var size = Vector2(self.rect_size.x/16,dest.rect_size.y)
			position_indicator.rect_size = size
			var position = Vector2(
				dest.get_position().x - size.x,
				dest.get_position().y
			)
			position_indicator.set_position(position)
			
func reposition_index_indicator():
	if index!= null and all_destinations.values().size() > 0:
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
	element.connect("set_active_destination",self,"set_active_destination")
	#reposition_index_indicator()

func delete_destination(uuid):
	emit_signal("delete_destination",uuid)
	
func set_active_destination(uuid):
	emit_signal("set_active_destination",uuid)
	
func destination_deleted(uuid):
	var dest_model = all_destinations[uuid]
	all_destinations.erase(uuid)
	self.remove_child(dest_model)
	dest_model.call_deferred('free')
	refresh_idexes()
	#refresh_destinations(all_destinations)
	
func add_destination_if_not_present(dest:Destination):
	if all_destinations.has(dest.uuid):
		pass
	else:
		add_destination(dest)
	#reposition_index_indicator()
	
func erase_destinations():
	for dest in all_destinations.values():
		self.remove_child(dest)
		if dest != null:
			dest.call_deferred("free")
			dest = null
	all_destinations = {}
	#position_indicator.visible = false

func erase_destinations_if_not_present(incoming_destinations:Dictionary):
	var incoming_uuid = {}
	for dest in incoming_destinations:
		if dest is Destination:
			incoming_uuid[dest.uuid] = dest
	for dest in all_destinations.values():
		if incoming_uuid.has(dest.uuid):
			pass
		else:
			all_destinations.erase(dest.uuid)
			self.remove_child(dest)
			if dest != null:
				dest.call_deferred("free")
				dest = null
			
	#position_indicator.visible = false
	
func refresh_destinations(destinations:Dictionary):
	#reposition_index_indicator()
	#erase_destinations()
	erase_destinations_if_not_present(destinations)
	for d in destinations.values():
		add_destination_if_not_present(d)
	#reposition_index_indicator()


func refresh_idexes():
	var dests = all_destinations.values()
	for i in range(0,dests.size()):
		var dest = dests[i]
		dest.index = i
		
func set_index(ind:int,uuid):
	index = ind
	if uuid == null:
		reposition_index_indicator()
	else:
		print_debug("set index" , uuid)
		reposition_index_by_id(uuid)
func _process(delta):
	reposition_index_indicator()
	self.rect_size = OS.window_size/4
	self.set_position(Vector2(
		OS.window_size.x - self.rect_size.x,
		0
	))
