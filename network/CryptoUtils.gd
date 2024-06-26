extends Node


# Declare member variables here. Examples:
# var a = 2
# var b = "text"


# Called when the node enters the scene tree for the first time.
func _ready():
	print(create_public_key().save_to_string(true))


func create_public_key() -> CryptoKey:
	var c = Crypto.new().generate_rsa(1024)
	return c
	
