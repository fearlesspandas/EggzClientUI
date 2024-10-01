extends Spatial

class_name ProwlerAnchor


onready var area:Area = Area.new()
onready var center:Spatial = Spatial.new()

var client_id:String

var npc_ids = []

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



func init_npcs():
	for id in npc_ids:
		socket.create_prowler(id,center.global_transform.origin)
		socket.get_blob(id)

func add_entity(npc:NPCServerEntity):
	self.add_child(npc)
	socket #make npc follow center
	


