extends Node



var entities = {}
func route(cmd):
	match cmd:
		{'NEW_ENTITY': {'id':var id,'location':var location, 'type': var type}}:
			EntityFactory.handle_message(cmd)
		{'SET_GLOB_LOCATION':{'id':var id,'location':var location}}:
			entities[id].find_node("MessageController").add_to_queue.append(cmd)
		{'SET_GLOB_ROTATION':{'id':var id, 'rotation':var rotation}}:
			entities[id].find_node("MessageController").add_to_queue.append(cmd)

# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
#func _process(delta):
#	pass
