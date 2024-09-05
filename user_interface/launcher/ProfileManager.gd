extends Node


var profiles = {}

var profiles_dir="user://profiles/"

func _ready():
	load_profiles()
	
func load_profiles():
	var dir = Directory.new()
	print("checking for profiles")
	if dir.open(profiles_dir) == OK:
		dir.list_dir_begin()
		var file_name = dir.get_next()
		while file_name != "":
			if dir.current_is_dir():
				print_debug("Found directory: " + file_name)
			else:
				var k = CryptoKey.new()
				assert(k.load(profiles_dir + file_name,false) == 0)
				if k.is_public_only():
					print_debug("Error: Key loaded that is public only: ",file_name)
					assert(false)
				var pp = PlayerProfile.new()
				pp.id = file_name.replace(".key","")
				pp.cryptoKey = k
				pp.file_location = profiles_dir + file_name
				profiles[file_name.replace(".key","")] = pp
			file_name = dir.get_next()
		dir.list_dir_end()
	else:
		print_debug("creating fresh profiles directory")
		dir.make_dir(profiles_dir)
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
	print_debug("Adding profile for ", id)
	if !profiles.has(id):
		Crypto.new().generate_rsa(1024).save(profiles_dir + id + ".key",false)
		load_profiles()
		return ERROR.OK
	else:
		return ERROR.PROFILE_EXISTS
		
func add_secret_to_profile(id,secret):
	var pp:PlayerProfile = get_profile(id)
	assert(pp != null)
	pp.set_secret_from_encrypted(secret)
	
enum ERROR { 
	OK = 0,
	PROFILE_EXISTS = 1
	}
