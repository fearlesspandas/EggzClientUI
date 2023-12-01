extends Node

class_name DestinationManager
signal new_dest(destination)
var destinations = []
var currentId = -1
var entity_spawn:Viewport


func add_destination(dest:Destination):
	destinations.append(dest)

func erase_dests():
	for dest in destinations:
		entity_spawn.remove_child(dest)
		if dest != null:
			dest.call_deferred("free")
			dest = null
	destinations = []
	
func _handle_message(dests):
	print("destinations:",destinations)
	erase_dests()
	for destination in dests:
		match destination:
			[var x,var y, var z]:
				var loc = Vector3(x,y,z)
				var newDest = Destination.new()
				newDest.location = Vector3(x,y,z)
				add_destination(newDest)
				print("adding new dest",destinations)
				entity_spawn.add_child(newDest)
	return destinations
