extends CheckButton


# Declare member variables here. Examples:
# var a = 2
# var b = "text"

onready var main:MainGame = find_parent("MainGame")
# Called when the node enters the scene tree for the first time.
func _ready():
	self.connect("toggled",self,"_on_SpawnServerEntities_toggled")
	pass # Replace with function body.


func _on_SpawnServerEntities_toggled(button_pressed):
	if button_pressed and ServerNetwork.socket.connected:
		main.spawn_server_world()
		main.create_character_entity_server("1")
	elif !ServerNetwork.socket.connected:
		print("Server Is not connected")
