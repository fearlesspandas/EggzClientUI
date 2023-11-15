extends TabContainer

class_name Profiles
var idx_map = {}
func _ready():
	pass # Replace with function body.

func create_profile_ui(profile:PlayerProfile):
	var res = PlayerProfileUI.new()
	res.profile = profile
	idx_map[profile.id] = self.get_tab_count()
	self.add_child(res)
	self.set_tab_title(idx_map[profile.id],profile.id)
