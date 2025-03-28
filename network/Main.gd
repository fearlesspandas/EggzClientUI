extends Node

func _ready():
	var profile = OS.get_environment("PROFILE")
	if profile == null or profile.length() == 0:
		var default = "CLIENT"
		print_debug("Error: no PROFILE environment variable set; using default ",default)
		
	match profile.to_upper():
		"SERVER":
			var host = OS.get_environment("EGGZ_HOST")
			#EGGZ_HOST checked because there is no gui input startup
			if host == null or host.length() == 0:
				print_debug("Error: No EGGZ_HOST environment variable set; using default ", NetworkConfig.host,NetworkConfig.physics_host)
			else:
				print_debug("Found EGGZ_HOST, using:" , host)
				NetworkConfig.host = host + ":8080"
				NetworkConfig.physics_host = host + ":8081"
			self.add_child(ServerBase.new())
		_:
			#EGGZ_HOST is not checked because it is input by the gui start screen
			self.add_child(Profiles.new())
		
