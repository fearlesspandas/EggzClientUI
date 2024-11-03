extends Spatial

#change to server specific
class_name ProwlerAnchorServer


onready var area:Area = Area.new()
onready var center:Spatial = Spatial.new()
onready var setup_timer : Timer = Timer.new()
onready var follow_timer: Timer = Timer.new()

var client_id:String

var npc_ids = [EntityTerrainMapper.generate_name(EntityTerrainMapper.NPCType.PROWLER)]

var npcs = {}



var radius:float = 1000

var socket:ClientWebSocket

var has_loaded:bool = false

var follow_body

func _ready():
	assert(EntityTerrainMapper.client_id_server != null)
	socket = ServerNetwork.get(EntityTerrainMapper.client_id_server)	
	assert(socket != null)

	self.add_child(center)
	center.global_transform.origin = self.global_transform.origin

	var collision_shape:CollisionShape = CollisionShape.new()
	var shape:BoxShape = BoxShape.new()
	shape.extents = Vector3(radius,radius,radius)
	collision_shape.shape = shape
	
	#var mesh:CubeMesh = CubeMesh.new()
	#mesh.size = 2*shape.extents
	#var mesh_instance = MeshInstance.new()
	#mesh_instance.mesh = mesh

	self.add_child(area)
	area.global_transform.origin = self.global_transform.origin
	#area.add_child(mesh_instance)
	area.add_child(collision_shape)
	area.set_collision_layer_bit(EntityConstants.SERVER_TERRAIN_COLLISION_LAYER,false)
	area.set_collision_mask_bit(EntityConstants.SERVER_TERRAIN_COLLISION_LAYER,false)
	area.set_collision_mask_bit(EntityConstants.SERVER_PLAYER_COLLISION_LAYER,true)
	area.connect("body_entered",self,"entered")
	area.connect("body_exited",self,"exited")

	setup_timer.wait_time = 1
	setup_timer.connect("timeout",self,"init_npcs")
	self.add_child(setup_timer)
	setup_timer.start()

	follow_timer.wait_time = 8
	follow_timer.connect("timeout",self,"follow_entity")
	self.add_child(follow_timer)
	follow_timer.start()
	follow_timer.set_paused(true)

	GlobalSignalsServer.connect("prowler_created",self,"init_prowler")
		

func init_prowler(id,prowler):
	if id in npc_ids:
		npcs[id] = prowler
		socket.set_destination_mode(id,"FORWARD")
		socket.clear_destinations(id)
		socket.add_destination(id,center.global_transform.origin,"WAYPOINT",1)
		socket.set_gravitate(id,true)
		#self.connect("exited",prowler)
	
func reset_to_center(id):
	socket.set_destination_mode(id,"FORWARD")
	socket.clear_destinations(id)
	socket.add_destination(id,center.global_transform.origin,"WAYPOINT",1)
	socket.set_gravitate(id,true)


func init_npcs():
	setup_timer.one_shot = true
	for id in npc_ids:
		print_debug("creating prowler, ", str(center.global_transform.origin + (Vector3.UP * radius)))
		socket.create_prowler(id,center.global_transform.origin + (Vector3.UP * radius))
		#socket.get_blob(id)

func follow_terrain():
	#necessary because location is not yet set during ready step
	for id in npc_ids:
		socket.set_destination_mode(id,"FORWARD")
		socket.clear_destinations(id)
		socket.add_destination(id,center.global_transform.origin,"WAYPOINT",1)
		socket.set_gravitate(id,true)

func follow_entity():
	assert(follow_body != null)
	for id in npc_ids:
		socket.set_gravitate(id,false)
		socket.follow_entity(id,follow_body)	

func entered(body):
	if body is ServerEntityKinematicBody:
		follow_body = body.parent.id
		follow_timer.set_paused(false)

func exited(body):
	if body is ServerEntityKinematicBody:
		for id in npc_ids:
			socket.unfollow_entity(id,body.parent.id)
			reset_to_center(id)
