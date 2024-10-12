extends StaticBody
class_name SpawnFrame

onready var setup_timer:Timer = Timer.new()

onready var socket:ClientWebSocket
# Called when the node enters the scene tree for the first time.
var npc_ids = ["Doxier"] #["Doxier","Heisenfire","Convvay","Neumenimum"]
func _ready():
	socket = ServerNetwork.get(EntityTerrainMapper.client_id_server)
	assert(socket != null)
	GlobalSignalsServer.connect("axis_spider_created",self,"spider_created")
	self.set_collision_layer_bit(EntityConstants.SERVER_TERRAIN_COLLISION_LAYER,true)
	self.set_collision_mask_bit(EntityConstants.SERVER_TERRAIN_COLLISION_LAYER,true)

	setup_timer.wait_time = 5
	setup_timer.connect("timeout",self,"setup")
	self.add_child(setup_timer)
	setup_timer.start()



func setup_path(id):
	socket.add_destination(id,self.global_transform.origin + Vector3(500,250,0),"WAYPOINT",1)
	socket.add_destination(id,self.global_transform.origin + Vector3(-500,250,0),"WAYPOINT",1)
	socket.add_destination(id,self.global_transform.origin + Vector3(500,-250,0),"WAYPOINT",1)
	socket.add_destination(id,self.global_transform.origin + Vector3(-500,-250,0),"WAYPOINT",1)
	socket.add_destination(id,self.global_transform.origin + Vector3(0,250,500),"WAYPOINT",1)
	socket.add_destination(id,self.global_transform.origin + Vector3(0,250,-500),"WAYPOINT",1)
	socket.add_destination(id,self.global_transform.origin + Vector3(0,-250,500),"WAYPOINT",1)
	socket.add_destination(id,self.global_transform.origin + Vector3(0,-250,-500),"WAYPOINT",1)


func setup():
	setup_timer.one_shot = true
	setup_timer.stop()
	for id in npc_ids:
		socket.create_axis_spider(id,self.global_transform.origin + Vector3(0,0,1000))
	

func spider_created(id,spider):
	pass
	#setup_path(id)
