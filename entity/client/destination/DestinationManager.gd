extends Node

class_name DestinationManager
signal new_destination(destination)
signal refresh_destinations(destinations)
signal clear_destinations()
signal index_set(next_index,uuid)
signal destination_deleted(uuid)

var client_id
var destinations = {}
var entity_spawn:Viewport
var index
var active_uuid

func _ready():
	assert(entity_spawn != null)
	
func make_destination_active(uuid):
	var res:Destination
	for i in range(0,destinations.values().size()):
		var dest = destinations.values()[i] 
		if dest is Destination and dest.uuid == uuid:
			res = dest
			print_debug(i,uuid)
			set_index(i,uuid)
	active_uuid = uuid

func get_active_destination() -> Destination:
	if active_uuid == null:
		return null
	else:
		return destinations[active_uuid]

func set_index(ind:int,uuid = null):
	index = ind
	emit_signal("index_set",index,uuid)
	GlobalSignalsClient.index_set(client_id,index,uuid)

func add_destination(dest:Destination):
	destinations[dest.uuid] = dest

func delete_destination(uuid:String):
	ServerNetwork.get(client_id).delete_destination(client_id,uuid)
	
func set_active_destination(uuid:String):
	ServerNetwork.get(client_id).set_active_destination(client_id,uuid)

func request_destination_refresh():
	ServerNetwork.get(client_id).get_all_destinations(client_id)

func destination_deleted(uuid:String):
	var dest = destinations[uuid]
	destinations.erase(uuid)
	entity_spawn.remove_child(dest)
	dest.call_deferred('free')
	if active_uuid == uuid:
		active_uuid = null
	emit_signal("destination_deleted",uuid)
	GlobalSignalsClient.destination_deleted(client_id,uuid)
	
func erase_dests():
	for dest in destinations.values():
		entity_spawn.remove_child(dest)
		if dest != null:
			dest.call_deferred("free")
			dest = null
	destinations = {}
	active_uuid = null
	
func spawn_dest(destination:Destination):
	entity_spawn.add_child(destination)
	
func destination(uuid:String,dest_type,location:Vector3,radius:float) -> Destination:
	var dest = Destination.new()
	dest.type = dest_type
	dest.location = location
	dest.radius = radius
	dest.uuid = uuid
	return dest
	
func handle_message(message):
	match message:
		{'ActiveDestination':{'id':var id, 'destination':var uuid}}:
			make_destination_active(uuid)
		{'DeleteDestination':{'id':var id, 'uuid':var uuid}}:
			destination_deleted(uuid)
		{'NextIndex':{'id':var id, 'index':var index}}:
			set_index(int(index))
		{'ClearDestinations':{}}:
			erase_dests()
			emit_signal("clear_destinations")
			GlobalSignalsClient.clear_destinations(client_id)
		{"AllDestinations":{"id":var id , "destinations":var dests}}:
			_handle_message(dests)
			emit_signal("refresh_destinations",destinations)
			GlobalSignalsClient.refresh_destinations(client_id,destinations)
		{'NewDestination':{'id':var id,'destination':{'uuid':var uuid, 'dest_type' : var dest_type,'location': [var x, var y ,var z],'radius':var radius}}}:
			var dest = destination(uuid,dest_type,Vector3(x,y,z),radius)
			add_destination(dest)
			spawn_dest(dest)
			emit_signal("new_destination",dest)
			GlobalSignalsClient.new_destination(client_id,dest)
			
func _handle_message(dests):
	erase_dests()
	for destination in dests:
		match destination:
			{'uuid':var uuid, 'dest_type':var dest_type ,'location':[var x,var y, var z] , 'radius':var radius}:
				var newDest = destination(uuid,dest_type,Vector3(x,y,z),radius)
				add_destination(newDest)
				spawn_dest(newDest)
			_:
				assert(false)
	return destinations

