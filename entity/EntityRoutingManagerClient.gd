extends Node



func parseJsonCmd(cmd,delta):
	#print("raw comand:",cmd)
	var parsed = JSON.parse(cmd)
	#print("errors:",parsed.error_string)
	if parsed.result != null:
		var json:Dictionary = parsed.result
		
		match json:
			{"NEW_ENTITY": {"id":var id,"location":var location, "type": var type}}:
				EntityFactory.handle_message(cmd,delta)
			{"NextDestination":{"id": var id, "location": [var x, var y , var z]}}:
				var s = EntityManager.server_entities[id]
				if s != null:
					var formatted = {"NextDestination":{"id":  id, "location": [ x, y , z]}}
					s.message_controller.add_to_queue(formatted)
			{"Location":{"id":var id, "location": [var x , var y , var z]}}:
				var s = EntityManager.client_entities[id]
				if s != null:
					var formatted = {"Location":{"id":id, "location": [ x , y , z]}}
					s.message_controller.add_to_queue(formatted)
			{"SET_GLOB_LOCATION":{"id":var id,"location":[var x, var y, var z]}}:
				var c = EntityManager.client_entities[id]
				if c != null:
					c.message_controller.add_to_queue(cmd)
				var s = EntityManager.server_entities[id]
				if s != null:
					s.message_controller.add_to_queue(cmd)
			{'SET_GLOB_ROTATION':{'id':var id, 'rotation':var rotation}}:
				var c = EntityManager.client_entities[id]
				if c != null:
					c.message_controller.add_to_queue(cmd)
				var s = EntityManager.server_entities[id]
				if s != null:
					s.message_controller.add_to_queue(cmd)
			_:
				print('No route specified for response:',cmd)
	else:
		pass
		#print("Could not parse msg:",cmd)

func route(cmd,delta):
	parseJsonCmd(cmd,delta)
	
	
# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
#func _process(delta):
#	pass
