extends TabContainer

class_name Profiles
onready var newProfile : NewProfileTab = NewProfileTab.new()

var idx_map = {}
var profiles = {}
func _ready():
	self.set_size(OS.get_window_safe_area().size,true)
	newProfile.boundaries = self.rect_size
	self.add_child(newProfile)
	newProfile.button.connect("button_up",self,"create_profile_from_input")
	self.connect("tab_changed",self,"on_tab_change")
	pass # Replace with function body.

func create_profile_ui(profile:PlayerProfile,clientProfile:bool):
	var res:Control
	print("is client:",clientProfile)
	if clientProfile:
		res = ClientControl.new()
	else:
		res = ServerControl.new()
	res.profile_id = profile.id
	idx_map[profile.id] = self.get_tab_count()
	profiles[profile.id] = res
	res.set_position(Vector2(0,0))
	res.set_size(self.rect_size)
	self.add_child(res)
	self.set_tab_title(idx_map[profile.id],profile.id)

func create_profile_from_input():
	var id = newProfile.textEdit.text
	var pp
	print_debug("profiles", ProfileManager.profiles)
	if ProfileManager.profile_exists(id):
		print_debug("found profile for ", id)
		pp = ProfileManager.get_profile(id)
	else:
		print_debug("creating new profile for ", id)
		if ProfileManager.add_profile(id) == 0:
			pp =  ProfileManager.get_profile(id)
	assert(pp != null)
	var isClient = !newProfile.checkbox.pressed
	create_profile_ui(pp,isClient)
	
func get_currently_selected_profile():
	var profile:Control = get_current_tab_control()
	return profile.profile


func on_tab_change(tab:int):
	var previous_tab_idx = get_previous_tab()
	var previous_tab = get_tab_control(previous_tab_idx)
	if previous_tab.has_method("set_active"):
		previous_tab.set_active(false)
	var active_tab = get_current_tab_control()
	if active_tab.has_method("set_active"):
		active_tab.set_active(true)

func _process(delta):
	if (self.rect_size - OS.get_window_safe_area().size).length() > 5:
		self.set_size(OS.get_window_safe_area().size,true)
