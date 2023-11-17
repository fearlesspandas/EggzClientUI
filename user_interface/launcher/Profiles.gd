extends TabContainer

class_name Profiles
onready var newProfile : NewProfileTab = NewProfileTab.new()

var idx_map = {}
var profiles = {}
func _ready():
	self.margin_top = 20
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
	res.profile = profile
	idx_map[profile.id] = self.get_tab_count()
	profiles[profile.id] = res
	res.set_position(Vector2(0,0))
	res.set_size(self.rect_size)
	self.add_child(res)
	
	
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


func on_tab_change(tab:int):
	var active_tab = get_current_tab_control()
	if active_tab.has_method("set_active"):
		active_tab.set_active(true)
	var all_tabs = profiles.values()
	for tab in all_tabs:
		if tab.has_method("set_active"):
			if active_tab.has_method("set_active") :
				if tab.profile.id != active_tab.profile.id:
					tab.set_active(false)
				else:
					tab.set_active(true)
			else:
				tab.set_active(false)
