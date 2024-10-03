extends Spatial

#change to server specific
class_name ProwlerAnchor


onready var area:Area = Area.new()
onready var center:Spatial = Spatial.new()
onready var follow_timer : Timer = Timer.new()

var client_id:String

var npc_ids = [EntityTerrainMapper.generate_name(EntityTerrainMapper.NPCType.PROWLER)]

var npcs = [] 


var radius:float = 50

var socket:ClientWebSocket

var has_loaded:bool = false

func _ready():
	assert(EntityTerrainMapper.client_id_server != null)
	socket = ServerNetwork.get(EntityTerrainMapper.client_id_server)	
	assert(socket != null)

	self.add_child(center)
	center.global_transform.origin = self.global_transform.origin

	var collision_shape:CollisionShape = CollisionShape.new()
	var shape:BoxShape = BoxShape.new()
	shape.extents = Vector3(radius,radius,radius)

	follow_timer.connect("timeout",self,"follow_terrain")
	follow_timer.wait_time = 2
	self.add_child(follow_timer)
	follow_timer.start()



func init_npcs():
	for id in npc_ids:
		print_debug("creating prowler, ", str(center.global_transform.origin + (Vector3.UP * radius)))
		socket.create_prowler(id,center.global_transform.origin + (Vector3.UP * radius))
		#socket.get_blob(id)

func add_entity(npc:NPCServerEntity):
	self.add_child(npc)
	socket #make npc follow center
	

func follow_terrain():
	#necessary because location is not yet set during ready step
	if not has_loaded:
		init_npcs()
		has_loaded = true
	else:
		for id in npc_ids:
			socket.set_destination_mode(id,"FORWARD")
			socket.clear_destinations(id)
			socket.add_destination(id,center.global_transform.origin,"WAYPOINT",1)
			socket.set_gravitate(id,true)
	

