extends Button


onready var main_game:MainGame = find_parent("MainGame")


# Called when the node enters the scene tree for the first time.
func _ready():
	self.connect("button_up",self,"create_new_profile_on_button_up")
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
#func _process(delta):
#	pass


func create_new_profile_on_button_up():
	var id = main_game.newProfileId.text
	main_game.file_manager.new_player_profile(id,"","")
	pass # Replace with function body.
