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
var is_empty = false

var collision_shape:CollisionShape = CollisionShape.new()
var shape:BoxShape = BoxShape.new()
var terrain_uuids = []
func _ready():
	self.input_ray_pickable = false
	shape.extents = Vector3(radius,radius,radius)
	mesh.size = 2*shape.extents
	mesh.material = SpatialMaterial.new()
	if is_empty:
		mesh.material.albedo_color = Color.red
	else:
		mesh.material.albedo_color = Color.darkgray
	mesh_instance.mesh = mesh
	#if self.is_empty:
	if !self.is_server:
		#self.add_child(mesh_instance)
		mesh_instance.visible = true
		
	collision_shape.shape = shape
	self.add_child(collision_shape)
	visibility_not.aabb = AABB(center - shape.extents,2*shape.extents)
	visibility_not.max_distance = radius
	visibility_not.connect("camera_entered",self,"chunk_is_visible")
	self.set_collision_mask_bit(0,false)
	self.set_collision_layer_bit(0,false)
	self.set_collision_mask_bit(10,true)
	self.set_collision_layer_bit(10,true)
	self.set_collision_layer_bit(11,true)
	self.set_collision_mask_bit(11,true)
	self.set_collision_layer_bit(12,true)
	self.set_collision_mask_bit(12,true)
	
	self.global_transform.origin = center
	#if self.is_empty:
		#print_debug("creating chunk ",center , " ", radius , " ", uuid)
	self.connect("body_entered",self,"body_entered_print")
	pass
	

func chunk_is_visible(camera:Camera):
	print_debug("Chunk IS VISIBLE ", uuid)
	#if !self.has_loaded:
		#load_terrain()
#func expand_chunk() -> Array:
	
func chunk_is_not_visible():
	pass
	#print_debug("Chunk is NOT visible ", uuid)
	
func body_entered_print(body):
	if body is KinematicBody:
		mesh_instance.visible = false
		if self.is_empty and self.is_server:
			if body is ServerEntityKinematicBody:
				print("body entered " + uuid+ " : " + str(body.parent.id))
				fill_empty_terrain(body.parent.id)
		else:
			load_terrain()
			self.has_loaded = true
	#if body is BlockTerrain:
		#terrain_uuids.push_back(body.uuid)
		#print(self.uuid + " " + str(terrain_uuids.size()))
		
func fill_empty_terrain(trigger_entity_id:String):
	#assert(false)
	if self.is_empty and not self.has_loaded:
		ServerNetwork.get(client_id).fill_empty_chunk(uuid,trigger_entity_id)
		#assert(false)
		self.has_loaded = true
		mesh_instance.visible = false

func load_terrain():
	assert(radius != 0)
	if !self.has_loaded and !self.is_empty:
		#print_debug("Loading Area " , center, " " ,radius , " ",uuid)
		ServerNetwork.get(client_id).get_cached_terrain(uuid)
		self.has_loaded = true
		var distance = ClientSettings.CHUNK_REQUEST_RADIUS_MULTIPLIER*radius
		ServerNetwork.get(client_id).get_top_level_terrain_in_distance(distance,center)
		#if is_empty:
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
