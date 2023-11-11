extends Spatial


# Declare member variables here. Examples:
# var a = 2
# var b = "text"

onready var camera_root = find_node("CameraRoot")

# Called when the node enters the scene tree for the first time.
func _ready():
	camera_root.global_transform.origin = self.global_transform.origin
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
#func _process(delta):
#	pass
