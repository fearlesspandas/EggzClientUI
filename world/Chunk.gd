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
onready var visibility_not : VisibilityNotifier = VisibilityNotifier.new()

var has_loaded:bool = false
var is_server = false


var player

var collision_shape:CollisionShape = CollisionShape.new()
var shape:BoxShape = BoxShape.new()

func _ready():
	shape.extents = Vector3(radius,radius,radius)
	collision_shape.shape = shape
	self.add_child(collision_shape)
	visibility_not.aabb = AABB(center - shape.extents,shape.extents * 2)
	visibility_not.max_distance = radius*2
	#visibility_not.connect("screen_entered",self,"chunk_is_visible")
	#visibility_not.connect("screen_exited",self,"chunk_is_not_visible")
	#self.add_child(visibility_not)
	self.set_collision_mask_bit(10,true)
	self.set_collision_mask_bit(0,false)
	self.set_collision_layer_bit(10,true)
	self.set_collision_layer_bit(0,false)
	self.global_transform.origin = center
	timer.wait_time = 3
	#timer.connect("timeout",self,"check_on_screen")
	self.add_child(timer)
	#timer.start()
	print_debug("creating chunk ",center , " ", radius , " ", uuid)
	self.connect("body_entered",self,"body_entered_print")
	pass
	

func chunk_is_visible():
	print_debug("Chunk IS VISIBLE ", uuid)
	if !self.has_loaded:
		load_terrain()
func chunk_is_not_visible():
	pass
	#print_debug("Chunk is NOT visible ", uuid)
	
func body_entered_print(body):
	#print_debug("BODY HAS ENTERED ", body)
	load_terrain()
	self.has_loaded = true
	
func load_terrain():
	if !self.has_loaded:
		print_debug("Loading Area " , center, " " ,radius , " ",uuid)
		ServerNetwork.get(client_id).get_cached_terrain(uuid)
		self.has_loaded = true
		ServerNetwork.get(client_id).get_top_level_terrain_in_distance(2*radius,center)
		#timer.stop()
func check_load():
	if !self.has_loaded:
		var entities = get_overlapping_bodies()
		if entities.size() > 0:
			load_terrain()
			self.has_loaded = true
