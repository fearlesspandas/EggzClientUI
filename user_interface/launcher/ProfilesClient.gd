extends Control

onready var profiles = Profiles.new()

# Called when the node enters the scene tree for the first time.
func _ready():
	
	self.add_child(profiles)
	profiles.set_size(self.rect_size)
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
#func _process(delta):
#	pass
