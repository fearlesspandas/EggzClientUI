extends Node


onready var body = find_node("PhysicalPlayerEntity")

export var id: String
func _physics_process(delta):
	ServerNetwork.setGlobLocation(id,body.global_transform.origin)
	pass
# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
#func _process(delta):
#	pass
