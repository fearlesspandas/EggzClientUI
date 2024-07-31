extends Area

class_name Chunk

var client_id
var uuid
var spawn
var center:Vector3
var radius:float
var entity_manager:EntityManagement
onready var timer:Timer = Timer.new()
onready var scheduler_timer:Timer = Timer.new()
var has_loaded:bool = false
var is_server = false
var is_listening = false
var player

var collision_shape:CollisionShape = CollisionShape.new()
var shape:BoxShape = BoxShape.new()

func _ready():
	shape.extents = Vector3(radius,radius,radius)
	collision_shape.shape = shape
	self.add_child(collision_shape)
	self.set_collision_mask_bit(10,true)
	self.set_collision_mask_bit(0,false)
	self.set_collision_layer_bit(10,true)
	self.set_collision_layer_bit(0,false)
	self.global_transform.origin = center
	timer.wait_time = 3
	timer.connect("timeout",self,"check_load")
	self.add_child(timer)
	timer.start()
	print_debug("creating chunk ",center , " ", radius , " ", uuid)
	pass
	
func load_terrain():
	if !self.has_loaded:
		print_debug("Loading Area " , center, " " ,radius , " ",uuid)
		ServerNetwork.get(client_id).get_cached_terrain(uuid)
		self.has_loaded = true
		ServerNetwork.get(client_id).get_top_level_terrain_in_distance(2*radius,center)

func check_load():
	if !self.has_loaded:
		var entities = get_overlapping_bodies()
		if entities.size() > 0:
			load_terrain()
			self.has_loaded = true
