extends HTTPRequest


class_name AuthenticationRequest

onready var main_game:MainGame = find_parent("MainGame")
# Called when the node enters the scene tree for the first time.
func _ready():
	self.connect("request_completed",self,"_on_request_completed")
	pass # Replace with function body.

func _initiate_auth_request(id:String):
	var url = NetworkConfig.get_verification_url(id)
	print("requesting url:", url)
	var headers = ["Content-Type: application/json"]
	print("request",self.request(url,headers))
# Called every frame. 'delta' is the elapsed time since the previous frame.
#func _process(delta):
#	pass


func _on_request_completed(result,responseCode,headers,body):
	var res = JSON.parse(body.get_string_from_utf8())
	match res.result:
		{"BasicSession":{"id":var id, "secret" : var secret}}:
			print(res.result)
			ServerNetwork.init(id,secret)
			main_game.file_manager.new_player_profile(id,secret,"myfilelocation")
		_:
			print("Authentication Http Requst could not find a handler for the result:",res.result)
