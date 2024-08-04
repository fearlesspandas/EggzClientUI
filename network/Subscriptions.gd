extends Node

var subs = {}

func subscribe(subscriber:String,to:String):
	subs[subscriber] = to

func get(subscriber:String):
	if subs.has(subscriber):
		return subs[subscriber]
	else:
		return null
var alphabet = "abcdefghijklmnopqrstuvwxyz"
var id_len = 16
func generate_id() -> String:
	var res = ""
	for i in range(0 , id_len):
		res += alphabet[int(rand_range(0,alphabet.length()))]
	#print_debug("subscriptions generated id: " , res)
	return res
