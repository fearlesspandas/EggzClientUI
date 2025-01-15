extends Spatial


var id:String
var client_id:String

onready var setup_timer:Timer = Timer.new()
var socket:ClientWebSocket = null
func _ready():
	assert(client_id != null and client_id.length() > 0)
	socket = ServerNetwork.get(client_id)
	assert(socket != null)

	setup_timer.connect("timeout",self,"setup")
	setup_timer.wait_time = 1
	self.add_child(setup_timer)
	setup_timer.start()
	
func init_with_id(id,client_id):
	self.id = id
	self.client_id = client_id

func setup():
	self.setup_timer.one_shot = true
	socket.create_monk_garden(id,self.global_transform.origin)

