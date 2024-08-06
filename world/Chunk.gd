extends Area

class_name Chunk

var client_id
var uuid
var spawn
var center:Vector3
var radius:float
onready var visibility_not : VisibilityNotifier = VisibilityNotifier.new()
onready var mesh_instance:MeshInstance = MeshInstance.new()
onready var mesh:CubeMesh = CubeMesh.new()
var has_loaded:bool = false
var is_server = false


var collision_shape:CollisionShape = CollisionShape.new()
var shape:BoxShape = BoxShape.new()

func _ready():
	shape.extents = Vector3(radius,radius,radius)
	mesh.size = 2*shape.extents #- Vector3(20,20,20)
	mesh.material = SpatialMaterial.new()
	mesh.material.albedo_color = Color.red
	mesh_instance.mesh = mesh
	self.add_child(mesh_instance)
	
	collision_shape.shape = shape
	self.add_child(collision_shape)
	visibility_not.aabb = AABB(center - shape.extents,2*shape.extents)
	visibility_not.max_distance = radius
	visibility_not.connect("camera_entered",self,"chunk_is_visible")
	self.set_collision_mask_bit(10,true)
	self.set_collision_mask_bit(0,false)
	self.set_collision_layer_bit(10,true)
	self.set_collision_layer_bit(0,false)
	self.set_collision_layer_bit(11,true)
	self.set_collision_mask_bit(11,true)
	self.global_transform.origin = center #- shape.extents
	mesh_instance.global_transform.origin = self.global_transform.origin #- shape.extents #center - shape.extents/2
#	print_debug("creating chunk ",center , " ", radius , " ", uuid)
	self.connect("body_entered",self,"body_entered_print")
	pass
	

func chunk_is_visible(camera:Camera):
	print_debug("Chunk IS VISIBLE ", uuid)
	#if !self.has_loaded:
		#load_terrain()

func chunk_is_not_visible():
	pass
	#print_debug("Chunk is NOT visible ", uuid)
	
func body_entered_print(body):
	load_terrain()
	self.has_loaded = true
	
func load_terrain():
	if !self.has_loaded:
		#print_debug("Loading Area " , center, " " ,radius , " ",uuid)
		ServerNetwork.get(client_id).get_cached_terrain(uuid)
		self.has_loaded = true
		ServerNetwork.get(client_id).get_top_level_terrain_in_distance(4*radius,center)
		mesh_instance.visible = false
		#timer.stop()
		
func check_load():
	if !self.has_loaded:
		var entities = get_overlapping_bodies()
		#if player != null and (player.global_transform.origin - center).length() < radius:
		if entities.size() > 0:
			load_terrain()
			self.has_loaded = true
			#timer.stop()

func is_within_chunk(loc:Vector3) -> bool:
	var xdiff = abs(loc.x - center.x)
	var ydiff = abs(loc.y - center.y)
	var zdiff = abs(loc.z - center.z)
	return xdiff <= radius and ydiff <= radius and zdiff <= radius

func is_within_distance(loc:Vector3,distance:float) -> bool:
	var diffs = loc - center
	var dist = distance + radius
	return abs(diffs.x) <= dist and abs(diffs.y) <= dist and abs(diffs.z) <= dist
