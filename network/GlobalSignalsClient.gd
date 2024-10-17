extends Node

#DESTINATIONS SIGNALS
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

#emits destination_hovered signal
signal destination_hovered(uuid)
func destination_hovered(uuid):
	emit_signal("destination_hovered",uuid)

#emits destination_unhovered signal
signal destination_unhovered(uuid)
func destination_unhovered(uuid):
	emit_signal("destination_unhovered",uuid)
#INVENTORY SIGNALS
#emits the item_added signal
signal item_added(client_id,item)
func item_added(client_id,item):
	emit_signal("item_added",client_id,item)

#emits the inventory signal (full inventory contents)
signal inventory(client_id,contents)
func inventory(client_id,contents):
	emit_signal("inventory",client_id,contents)

#PLAYER SIGNALS
#emits player_position signal
signal player_position(location)
func player_position(location):
	emit_signal("player_position",location)

#INPUT SIGNALS
#emits activate_ability signal
signal activate_ability(client_id,slot_id)
func activate_ability(client_id,slot_id):
	emit_signal("activate_ability",client_id,slot_id)
	

#TERRAIN SIGNALS
#emits spawn_node signal
signal spawn_node(node,location)
func spawn_node(node,location):
	emit_signal("spawn_node",node,location)
