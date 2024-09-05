extends Node

func _ready():
	var profile = OS.get_environment("PROFILE")
	if profile == null or profile.length() == 0:
		var default = "CLIENT"
		print_debug("Error: no PROFILE environment variable set; using default ",default)
	match profile.to_upper():
		"SERVER":
			self.add_child(ServerBase.new())
		_:
			self.add_child(Profiles.new())
		
