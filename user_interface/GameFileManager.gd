extends Node
signal profile_created(profile)
class_name GameFileManager
var profiles = {}
func new_player_profile(id:String,secret:String,file_location:String) -> PlayerProfile:
	var res = PlayerProfile.new()
	res.id = id
	res.secret = secret
	res.file_location = file_location
	profiles[id] = res
	print("created profile", res.id)
	emit_signal("profile_created",res)
	return res
func get_player_profile():
	pass

func _ready():
	pass # Replace with function body.


