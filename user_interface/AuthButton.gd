extends Button


onready var main_game : MainGame = find_parent("MainGame")

# Called when the node enters the scene tree for the first time.
func _ready():
	self.connect("button_up",self,"_on_authbutton_button_up")


func _on_authbutton_button_up():
	
	var profile:PlayerProfileUI = main_game.profiles.get_current_tab_control()
	print("calling button up in auth")
	if profile != null:
		
		var id = profile.profile.id
		print("authorizing with id ",id)
		main_game.authrequest._initiate_auth_request(id)
	else:
		print("select a profile to connect to the server")
