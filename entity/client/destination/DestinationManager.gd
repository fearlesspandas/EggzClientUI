extends Node

class_name DestinationManager
signal new_dest(destination)
var destinations = {}
var currentId = -1
var entity_spawn:Viewport


func add_destination(dest:Destination,loc):
	destinations[loc] = dest

func _handle_message(dests):
	var updated = []
	for destination in dests:
		match destination:
			[var x,var y, var z]:
				var loc = Vector3(x,y,z)
				if !destination.has(loc):
					var newDest = Destination.new()
					newDest.location = Vector3(x,y,z)
					add_destination(newDest,[x,y,z])
					print("adding new dest",destinations)
					entity_spawn.add_child(newDest)
				updated.append([x,y,z])
	for l in destinations.keys():
		if !updated.has(l):
			print("erasing child")
			entity_spawn.remove_child(destinations[l])
			destinations.erase(l)
	return destinations
