extends HTTPRequest

signal session_created(id,secret)

class_name AuthenticationRequest

func _ready():
	self.connect("request_completed",self,"_on_request_completed")
	pass # Replace with function body.

func _initiate_auth_request(pp_id:String):
	var pp = ProfileManager.get_profile(pp_id)
	assert(pp != null, "No profile found for id " + pp_id)
	var url = NetworkConfig.get_verification_url(pp.id)
	print_debug("requesting url:", url)
	var headers = ["Content-Type: application/json"]
	#print("request",self.request(url,headers))
	if pp.cryptoKey == null:
		print_debug("Error: Public key is null!")
	var key = pp.cryptoKey.save_to_string(true)
	print_debug("Authenticating with key: ",key)
	print_debug("request",self.request(url,headers,true,HTTPClient.METHOD_GET,key))



func _on_request_completed(result,responseCode,headers,body):
	var res = JSON.parse(body.get_string_from_utf8())
	print_debug("Response received, processing response...")
	match res.result:
		{"BasicSession":{"id":var id, "secret" : var secret}}:
			print_debug("HttpAuthResult:",res.result)
			ProfileManager.add_secret_to_profile(id,secret)
			emit_signal("session_created",id,secret)
		_:
			print_debug("Authentication Http Requst could not find a handler for the result:",res.result)
