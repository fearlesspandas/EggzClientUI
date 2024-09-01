extends Command

class_name RequestTopTerrainInDistance

var client_id:String

func _ready():
	assert(client_id != null and client_id.length() > 0)
	self.connect("button_clicked",self,"send_request")


func send_request(argmap):
	var radius = float(argmap["radius"])
	var player:Player = DataCache.cached(client_id,"PLAYER")
	var location = player.body.global_transform.origin
	ServerNetwork.get(client_id).get_top_level_terrain_in_distance(radius,location)
	print_debug("request sent with args " + str(argmap))
