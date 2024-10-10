extends Node

#emits new_destination signal
signal new_destination(destination)
func new_destination(destination):
	emit_signal("new_destination",destination)

#emits refresh_destinations signal 
signal refresh_destinations(destinations)
func refresh_destinations(destinations):
	emit_signal("refresh_destinations",destinations)

#emits the clear_destinations signal
signal clear_destinations()
func clear_destinations():
	emit_signal("clear_destinations")

#emits the index_set signal
signal index_set(next_index,uuid)
func index_set(next_index,uuid):
	emit_signal("index_set",next_index,uuid)

#emits the destination_deleted signal
signal destination_deleted(uuid)
func destination_deleted(uuid):
	emit_signal("destination_deleted",uuid)
