extends Node

enum {COMMAND}
var current_tree = {
	"GET_GLOB_LOCATION": {"id":TYPE_STRING},
	"SET_GLOB_LOCATION": {"id":TYPE_STRING,"location":TYPE_VECTOR3_ARRAY},
	"SUBSCRIBE":{"id":TYPE_STRING,"query":COMMAND}
}
var cmd = ""
func parse(cmd):
	match cmd:
		"GET_GLOB_LOCATION":
			pass
		"SET"	:
			pass
	pass
# Called when the node enters the scene tree for the first time.


func parse_text(cmd) -> String:
	var split = cmd.split(" ")
	match split.size():
		0:
			
			return ""
		1:
			return ""
		2:
			var head = split[0]
			var args_split = split.remove(0)
			match head: 
				"SET_GLOB_LOCATION":
					var loc = Vector3(args_split[0],args_split[1],args_split[2])
					var msg = PayloadMapper.setGlobLocation(args_split[0],loc)
					return msg
				_:
					return ""
		_:
			return ""
