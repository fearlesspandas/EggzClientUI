extends Node
export var spawnWorld :Resource

# Declare member variables here. Examples:
# var a = 2
# var b = "text"


# Called when the node enters the scene tree for the first time.
func _ready():
	var spawn = load(spawnWorld.resource_path).instance()
	self.add_child(spawn)
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
#func _process(delta):
#	pass
