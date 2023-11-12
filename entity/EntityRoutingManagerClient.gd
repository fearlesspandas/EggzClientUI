extends Node



var entities = {}
func route(cmd,delta):
	match cmd:
		{'NEW_ENTITY': {'id':var id,'location':var location, 'type': var type}}:
			EntityFactory.handle_message(cmd,delta)
		{'SET_GLOB_LOCATION':{'id':var id,'location':var location}}:
			entities[id].message_controller.add_to_queue(cmd)
		{'SET_GLOB_ROTATION':{'id':var id, 'rotation':var rotation}}:
			entities[id].message_controller.add_to_queue(cmd)

# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
#func _process(delta):
#	pass
