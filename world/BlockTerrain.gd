extends StaticBody
onready var parent = get_parent()

func _ready():
	connect("mouse_entered",self,"glow")

func glow():
	if parent != null:
		pass
