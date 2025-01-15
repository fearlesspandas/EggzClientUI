extends Node

var data = {} 

func get(id) -> Array:
	if data.has(id):
		return data[id]
	else:
		return [0,1]

func contains(id) -> bool:
	return data.has(id)

func add(id,items):
	data[id] = items
