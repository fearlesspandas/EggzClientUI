extends Control

export var isclient:bool
onready var clientControl:ClientControl = ClientControl.new()
onready var serverControl:ServerControl = ServerControl.new()
var client_dims:Vector2
var client_id
class_name MainGameScene
#starting server means loading server entities which will automatically
#start updating with message traffic
#similarly starting a client is just instantiating spawn map currenly

func prep_client():
	#clientControl.set_global_position(client_dims)
	#clientControl.set_size(dims.size)
	clientControl.set_global_position(Vector2(0,0))
	#var pp = PlayerProfile.new()
	#pp.id = "1"
	#pp.secret = "SECRET"
	#pp.file_location = ""
	#clientControl.profile = pp
	self.add_child(clientControl)
func prep_server():
	serverControl.set_global_position(client_dims)
	#clientControl.set_size(dims.size)
	serverControl.set_global_position(Vector2(0,0))
	self.add_child(serverControl)
func _ready():
	if isclient:
		prep_client()
	else:
		prep_server()
	print("readying main")
	pass

