extends Area

class_name Chunk

var client_id
var uuid
var spawn
var center:Vector3
var radius:float
onready var visibility_not : VisibilityNotifier = VisibilityNotifier.new()
onready var mesh_instance:MeshInstance = MeshInstance.new()

var has_loaded:bool = false
var is_server = false
var is_empty = false

var collision_shape:CollisionShape = CollisionShape.new()
var shape:BoxShape = BoxShape.new()
var terrain_uuids = []
var should_spawn_texture:bool = true
func _ready():
	self.input_ray_pickable = false
	shape.extents = Vector3(radius,radius,radius)
	if (should_spawn_texture or is_empty) and !self.is_server :
		var mesh:CubeMesh = CubeMesh.new()
		mesh.size = 2*shape.extents - Vector3(20,20,20)
		var material = SpatialMaterial.new()
		mesh.material = material
		if is_empty:
			mesh.material.albedo_color = Color.red
		else:
			if should_spawn_texture:
				#var starfield_texture:StreamTexture = load("res://textures/vortex.png")
				#starfield_texture.flags = 7
				#material.albedo_texture = starfield_texture
				mesh.material.albedo_color = Color.black
				#mesh.material.albedo_color.a = 1
				#mesh.material.albedo_color.v = 1
				#mesh.material.albedo_color.s = 0.5
		mesh_instance.mesh = mesh
		if !self.is_server:
			self.add_child(mesh_instance)
			mesh_instance.visible = DataCache.cached_with("CLIENT","chunks_visible",true)
		
	collision_shape.shape = shape
	self.add_child(collision_shape)
	visibility_not.aabb = AABB(center - shape.extents,2*shape.extents)
	visibility_not.max_distance = radius
	visibility_not.connect("camera_entered",self,"chunk_is_visible")
	self.set_collision_mask_bit(EntityConstants.SERVER_TERRAIN_COLLISION_LAYER,false)
	self.set_collision_layer_bit(EntityConstants.SERVER_TERRAIN_COLLISION_LAYER,false)
	self.set_collision_mask_bit(EntityConstants.SERVER_PLAYER_COLLISION_LAYER,true)
	self.set_collision_layer_bit(EntityConstants.SERVER_PLAYER_COLLISION_LAYER,false)
	self.set_collision_layer_bit(EntityConstants.CLIENT_PLAYER_COLLISION_LAYER,true)
	self.set_collision_mask_bit(EntityConstants.CLIENT_PLAYER_COLLISION_LAYER,true)
	self.set_collision_layer_bit(EntityConstants.SERVER_NPC_COLLISION_LAYER,false)
	self.set_collision_mask_bit(EntityConstants.SERVER_NPC_COLLISION_LAYER,false)
	
	self.global_transform.origin = center
	#if self.is_empty:
		#print_debug("creating chunk ",center , " ", radius , " ", uuid)
	self.connect("body_entered",self,"body_entered_print")
	pass
	

func toggle_chunk_visibility(is_visible):
	if self.mesh_instance !=null and !has_loaded:
		self.mesh_instance.visible = is_visible

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
