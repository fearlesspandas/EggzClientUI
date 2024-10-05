extends Command

class_name GetAllEntities

var client_id:String


func _init():
	self.command_name = "get all entities"


func _ready():
	assert(client_id != null and client_id.length() > 0)
	self.connect("button_clicked",self,"send_request")


func send_request(argmap):
	ServerNetwork.get(client_id).getAllGlobs()	



