extends Node

class_name DestinationManager
signal new_destination(destination)
signal refresh_destinations(destinations)
signal clear_destinations()
signal index_set(next_index)


var client_id
var destinations = []
var entity_spawn:Viewport
var index

func _ready():
	assert(entity_spawn != null)
	
func poll_index():
	ServerNetwork.get(client_id).get_next_destination_index(client_id)
	
#empty string id is a shorthand for global access
func set_index(ind:int):
	index = ind
	emit_signal("index_set",index)
	#DataCache.add_data("","index",index)
	
func add_destination(dest:Destination):
	destinations.append(dest)
	
func erase_dests():
	for dest in destinations:
		entity_spawn.remove_child(dest)
		if dest != null:
			dest.call_deferred("free")
			dest = null
	destinations = []
	
func spawn_dest(destination:Destination):
	entity_spawn.add_child(destination)
	
func destination(dest_type,location:Vector3,radius:float) -> Destination:
	var dest = Destination.new()
	dest.type = dest_type
	dest.location = location
	dest.radius = radius
	return dest
	
func handle_message(message):
	match message:
		{'NextIndex':{'id':var id, 'index':var index}}:
			set_index(int(index))
		{'ClearDestinations':{}}:
			erase_dests()
			emit_signal("clear_destinations")
		{"AllDestinations":{"id":var id , "destinations":var dests}}:
			_handle_message(dests)
			emit_signal("refresh_destinations",destinations)
		{'NewDestination':{'id':var id,'destination':{'dest_type' : var dest_type,'location': [var x, var y ,var z],'radius':var radius}}}:
			var dest = destination(dest_type,Vector3(x,y,z),radius)
			add_destination(dest)
			spawn_dest(dest)
			emit_signal("new_destination",dest)
			
func _handle_message(dests):
	erase_dests()
	for destination in dests:
		match destination:
			{'dest_type':var dest_type ,'location':[var x,var y, var z] , 'radius':var radius}:
				var newDest = destination(dest_type,Vector3(x,y,z),radius)
				add_destination(newDest)
				spawn_dest(newDest)
	return destinations

