extends Area

class_name GlobularTeleportServer

var points = []
var base:Vector3

func _ready():
	convex_ready()

	self.set_collision_layer_bit(EntityConstants.SERVER_TERRAIN_COLLISION_LAYER,false)
	self.set_collision_mask_bit(EntityConstants.SERVER_TERRAIN_COLLISION_LAYER,false)

	self.set_collision_layer_bit(EntityConstants.SERVER_PLAYER_COLLISION_LAYER,true)
	self.set_collision_mask_bit(EntityConstants.SERVER_PLAYER_COLLISION_LAYER,true)

	self.set_collision_layer_bit(EntityConstants.SERVER_BOSS_COLLISION_LAYER,false)
	self.set_collision_mask_bit(EntityConstants.SERVER_BOSS_COLLISION_LAYER,true)

	self.connect("body_entered",self,"body_entered")
	self.connect("body_exited",self,"body_exited")

func convex_ready():
	assert(points.size() > 0)
	assert(base != null)
	var shape: ConvexPolygonShape = ConvexPolygonShape.new()
	shape.points = points
	var collision_shape = CollisionShape.new()
	collision_shape.shape = shape
	self.add_child(collision_shape)
	self.global_transform.origin = base



func sphere_ready():
	assert(points.size() >= 2)
	assert(base != null)
	var shape: SphereShape = SphereShape.new()
	shape.radius = (points[0] - points[1]).length()
	var collision_shape = CollisionShape.new()
	collision_shape.shape = shape
	self.add_child(collision_shape)
	self.global_transform.origin = base + points[0]



func body_entered(body):
	if body is ServerEntityKinematicBody:
		print_debug("GlobularTeleport entered: ", body.global_transform.origin)
		body.global_transform.origin = base
	

func body_exited(body):
	print_debug("GlobularTeleport exited: ", body.global_transform.origin)


	

