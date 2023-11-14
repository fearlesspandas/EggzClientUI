extends Node

class_name MessageController
onready var parent = get_parent()
var queue = []
var queue_out = [] #queue of events to be sent back to the server
var delta_accum = 0
func add_to_queue(elem):
	queue.append(elem)
func handle_next_message(delta):
	if queue.size() > 0:
		delta_accum += delta
		var msg = queue.pop_front()
		if parent.has_method("_handle_message"):
			parent._handle_message(msg,delta_accum)
			delta_accum = 0
		else:
			queue.push_front(msg)
			print("no handler found on resource parent: define method _handle_method on this nodes parent to fix this")
	else:
		pass
		#print("no messages in queue, waiting")
		#todo potentially replace this delta with custom delta from last processed message
func _physics_process(delta):
	handle_next_message(delta)

# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
#func _process(delta):
#	pass
