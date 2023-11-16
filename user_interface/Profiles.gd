extends TabContainer

class_name Profiles
var idx_map = {}
var profiles = {}
func _ready():
	pass # Replace with function body.

func create_profile_ui(profile:PlayerProfile):
	var res = PlayerProfileUI.new()
	res.profile = profile
	idx_map[profile.id] = self.get_tab_count()
	profiles[profile.id] = profile
	self.add_child(res)
	self.set_tab_title(idx_map[profile.id],profile.id)


func get_currently_selected_profile():
	var profile:PlayerProfileUI = profiles.get_current_tab_control()
	return profile.profile
