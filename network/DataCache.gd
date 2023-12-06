#Data cache serves as an easy reference service for data that isn't 
#extremely high throughput. Don't use for physics dependent updates
#frame by frame for things like setting real time location
#data that comes in at a rate > .1 seconds is probably fine

extends Node


var store = {}


func add_data(client_id:String,field:String, val):
	if store.has(client_id):
		store[client_id][field] = val
	else:
		store[client_id] = {field:val}
func remove_data(client_id:String,field:String):
	if store.has(client_id):
		store[client_id].erase(field)
func cached(id:String,field:String):
	if store.has(id) and store[id].has(field):
		return store[id][field]
	else:
		return null
