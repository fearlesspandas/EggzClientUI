extends Command

class_name CreateProwler


var client_id:String


func _init():
	self.command_name = "create prowler"
	self.add_args(["name","x","y","z"])

func _ready():
	assert(client_id != null and client_id.length() > 0)
	self.connect("button_clicked",self,"send_request")

func send_request(argmap):
	var id = argmap["name"]
	var loc:Vector3 = Vector3()
	loc.x = float(argmap["x"])
	loc.y = float(argmap["y"])
	loc.z = float(argmap["z"])
	ServerNetwork.get(client_id).create_prowler(id,loc)
	print_debug("Creating prowler ",id, " at ",loc)
