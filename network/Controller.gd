extends Node
#############delete this
export var message_controller_resource:Resource = preload("res://network/MessageController.tscn")
func new():
	var controller = load(message_controller_resource.resource_path).instance()
	return controller	
# Declare member variables here. Examples:
# var a = 2
# var b = "text"


# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
#func _process(delta):
#	pass
