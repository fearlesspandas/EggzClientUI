extends Node


class_name PlayerProfile


var username:String
var id:String
var secret:String
var file_location:String
var cryptoKey : CryptoKey



# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.



func set_secret_from_encrypted(encrypted_secret:String):
	print_debug("profile ", id, " is public only:" ,cryptoKey.is_public_only())
	secret =  Crypto.new().decrypt(cryptoKey,Marshalls.base64_to_raw(encrypted_secret)).get_string_from_utf8()
	print_debug("Secret set for profile ", id, " secret:", secret)
