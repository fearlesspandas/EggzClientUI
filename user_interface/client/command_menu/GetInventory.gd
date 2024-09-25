extends Command
class_name GetInventory

var client_id:String

func _init():
	self.command_name = "Get Inventory"

func _ready():
	assert(client_id != null and client_id.length() > 0)
	self.connect("button_clicked",self,"request_inventory")

func request_inventory(argmap):
	ServerNetwork.get(client_id).get_inventory(client_id)
