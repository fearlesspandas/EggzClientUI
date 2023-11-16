extends TabContainer

class_name Profiles
onready var newProfile : NewProfileTab = NewProfileTab.new()

var idx_map = {}
var profiles = {}
func _ready():
	self.set_size(OS.get_window_safe_area().size)
	newProfile.boundaries = self.rect_size
	self.add_child(newProfile)
	newProfile.button.connect("button_up",self,"create_profile_from_input")
	pass # Replace with function body.

func create_profile_ui(profile:PlayerProfile,clientProfile:bool):
	var res:Control
	print("is client:",clientProfile)
	if clientProfile:
		res = ClientControl.new()
	else:
		res = ServerControl.new()
	res.profile = profile
	idx_map[profile.id] = self.get_tab_count()
	profiles[profile.id] = profile
	self.add_child(res)
	res.set_size(self.rect_size)
	res.set_global_position(Vector2(0,0))
	self.set_tab_title(idx_map[profile.id],profile.id)

func create_profile_from_input():
	var id = newProfile.textEdit.text
	var pp = PlayerProfile.new()
	pp.id = id
	var isClient = !newProfile.checkbox.pressed
	create_profile_ui(pp,isClient)
	
func get_currently_selected_profile():
	var profile:Control = get_current_tab_control()
	return profile.profile
