extends Node


var profiles = {}


func _ready():
	load_profiles()
	
func load_profiles():
	var dir = Directory.new()
	print("checking for profiles")
	pass
	if dir.open("res://profiles/") == OK:	
		dir.list_dir_begin()
		var file_name = dir.get_next()
		while file_name != "":
			if dir.current_is_dir():
				print_debug("Found directory: " + file_name)
			else:
				var k = CryptoKey.new()
				assert(k.load("res://profiles/" + file_name,false) == 0)
				var pp = PlayerProfile.new()
				pp.id = file_name.replace(".key","")
				pp.cryptoKey = k
				pp.file_location = "res://profiles/" + file_name
				profiles[file_name.replace(".key","")] = pp
			file_name = dir.get_next()
		dir.list_dir_end()
	else:
		print_debug("creating fresh profiles directory")
		dir.make_dir("res://profiles/")
	print_debug("loaded profiles: ",profiles.size())
	#for p in profiles:
		#print_debug(get_profile(p).id)

func profile_exists(id:String) -> bool:
	return profiles.has(id)

func get_profile(id:String) -> PlayerProfile:
	if profiles.has(id):
		return profiles[id]
	else:
		return null
func get_all_profiles() -> Array:
	return profiles.values()
func add_profile(id:String) -> int:
	if !profiles.has(id):
		Crypto.new().generate_rsa(1024).save("res://profiles/" + id + ".key",false)
		load_profiles()
		return ERROR.OK
	else:
		return ERROR.PROFILE_EXISTS
		
func add_secret_to_profile(id,secret):
	var pp = get_profile(id)
	assert(pp != null)
	pp.set_secret_from_encrypted(secret)
	
enum ERROR { 
	OK = 0,
	PROFILE_EXISTS = 1
	}
