extends Spatial

#change to server specific
class_name ProwlerAnchor


onready var area:Area = Area.new()
onready var center:Spatial = Spatial.new()
onready var follow_timer : Timer = Timer.new()

var client_id:String

var npc_ids = [EntityTerrainMapper.generate_name(EntityTerrainMapper.NPCType.Prowler)]

var npcs = [] 


var radius:float

var socket:ClientWebSocket

func _ready():
	socket = ServerNetwork.get(client_id)	
	assert(socket != null)
	init_npcs()

	self.add_child(center)

	var collision_shape:CollisionShape = CollisionShape.new()
	var shape:BoxShape = BoxShape.new()
	shape.extents = Vector3(radius,radius,radius)

	follow_timer.connect("timeout",self,"follow_terrain")
	follow_timer.wait_time = 2
	self.add_child(follow_timer)
	follow_timer.start()



func init_npcs():
	for id in npc_ids:
		socket.create_prowler(id,center.global_transform.origin)
		#socket.get_blob(id)

func add_entity(npc:NPCServerEntity):
	self.add_child(npc)
	socket #make npc follow center
	

func follow_terrain():
	for id in npc_ids:
		socket.clear_destinations(id)
		socket.add_destination(id,center.global_transform.origin,"WAYPOINT",1)
		socket.set_gravitate(id,true)
	


