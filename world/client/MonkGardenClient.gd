extends Spatial

var client_id
var id 

var socket:ClientWebSocket = null


onready var body = find_node("Body")
onready var base = find_node("Base")
onready var leaf1 = find_node("Leaf1")
onready var leaf2 = find_node("Leaf2")
onready var leaf3 = find_node("Leaf3")
onready var leaf4 = find_node("Leaf4")
onready var collision_object = find_node("CollisionObject")
onready var area = find_node("UseableArea")

var player_active = false
var initialized = false
var items = []


func _ready():
	assert(client_id != null and client_id.length() > 0)
	assert(id != null and id.length() > 0)
	socket = ServerNetwork.get(client_id)
	assert(socket != null)
	GlobalSignalsClient.connect("monk_garden_received",self,"initialize")
	set_colliders()
	set_signal_handlers()

func initialize(id,location,items):
	if id == self.id:
		print_debug("MonkGarden Received ",id," with items ", items)
		initialized = true
		self.items = items

func set_colliders():
	collision_object.set_collision_layer_bit(EntityConstants.SERVER_TERRAIN_COLLISION_LAYER,false)
	collision_object.set_collision_mask_bit(EntityConstants.SERVER_TERRAIN_COLLISION_LAYER,false)
	collision_object.set_collision_layer_bit(EntityConstants.CLIENT_PLAYER_COLLISION_LAYER,true)
	collision_object.set_collision_mask_bit(EntityConstants.CLIENT_PLAYER_COLLISION_LAYER,true)
	area.set_collision_layer_bit(EntityConstants.SERVER_TERRAIN_COLLISION_LAYER,false)
	area.set_collision_mask_bit(EntityConstants.SERVER_TERRAIN_COLLISION_LAYER,false)
	area.set_collision_mask_bit(EntityConstants.CLIENT_SHOPS_COLLISION_LAYER,true)

func set_signal_handlers():
	area.connect("body_entered",self,"entered")
	area.connect("body_exited",self,"exited")

func entered(body):
	player_active = true
	print_debug("body entered",str(body))
	print_debug("monk items ", self.items)
	assert(initialized)
	for item in self.items:
		ShopMenuEnv.add_to_inventory(int(item))

func exited(body):
	player_active = false
	print_debug("body exited", str(body))
	ShopMenuEnv.clear_inventory()

func _physics_process(delta):
	if player_active:
		leaf1.global_rotation.y += delta
		leaf2.global_rotation.y -= delta
		leaf3.global_rotation.y += delta
		leaf4.global_rotation.y -= delta

func init_with_id(id,client_id):
	self.id = id
	self.client_id = client_id


