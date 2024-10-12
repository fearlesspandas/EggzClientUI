extends Node

#emits new_destination signal
signal new_destination(client_id,destination)
func new_destination(client_id,destination):
	emit_signal("new_destination",client_id,destination)

#emits refresh_destinations signal 
signal refresh_destinations(client_id,destinations)
func refresh_destinations(client_id,destinations):
	emit_signal("refresh_destinations",client_id,destinations)

#emits the clear_destinations signal
signal clear_destinations()
func clear_destinations(client_id):
	emit_signal("clear_destinations",client_id)

#emits the index_set signal
signal index_set(client_id,next_index,uuid)
func index_set(client_id,next_index,uuid):
	emit_signal("index_set",client_id,next_index,uuid)

#emits the destination_deleted signal
signal destination_deleted(client_id,uuid)
func destination_deleted(client_id,uuid):
	emit_signal("destination_deleted",client_id,uuid)

#emits the item_added signal
signal item_added(client_id,item)
func item_added(client_id,item):
	emit_signal("item_added",client_id,item)

#emits player_position signal
signal player_position(location)
func player_position(location):
	emit_signal("player_position",location)
