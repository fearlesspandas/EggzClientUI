extends Node


# Declare member variables here. Examples:
# var a = 2
# var b = "text"
onready var httpRequest:HTTPRequest = find_node("HttpRequest")
var id
var public_key

# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.

func authenticate(id):
	pass

onready var verified_session_tokens = {}


func verify_session_token(publickey,token,retryCount = 0) -> bool:
	print("checking")
	if verified_session_tokens.has(publickey):
		print("has")
		return (verified_session_tokens.get(publickey) == token)
	else:
		return false
			
remote func initialize_session_token(publickey,token):
	http_query_session_token(publickey,token)
	
func http_query_session_token(publickey,token):
	var query = JSON.print({"publickey":publickey,"token":token})
	httpRequest.request(HttpServerManager.url + "/verify_session",HttpServerManager.headers,HttpServerManager.use_ssl,HTTPClient.METHOD_POST,query)
	
func _on_HTTPRequest_request_completed(result, response_code, headers, body):
	var res = body.get_string_from_utf8()
	var json = res.to_json()
	print("json" + json)
	if bool(json.valid):
		verified_session_tokens[json.publickey] = true
