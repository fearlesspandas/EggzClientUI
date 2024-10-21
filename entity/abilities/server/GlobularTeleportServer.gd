extends Area

class_name GlobularTeleportServer

var points = []
var base:Vector3

func _ready():

	assert(points.size() > 0)
	assert(base != null)
	self.connect("body_entered",self,"body_entered")
	self.connect("body_exited",self,"body_exited")

	var shape: ConvexPolygonShape = ConvexPolygonShape.new()
	shape.points = points
	var collision_shape = CollisionShape.new()
	collision_shape.shape = shape
	self.add_child(collision_shape)
	self.global_transform.origin = base

	self.set_collision_layer_bit(EntityConstants.SERVER_TERRAIN_COLLISION_LAYER,false)
	self.set_collision_mask_bit(EntityConstants.SERVER_TERRAIN_COLLISION_LAYER,false)

	self.set_collision_layer_bit(EntityConstants.SERVER_PLAYER_COLLISION_LAYER,true)
	self.set_collision_mask_bit(EntityConstants.SERVER_PLAYER_COLLISION_LAYER,true)



func sphere_ready():
	assert(points.size() >= 2)
	assert(base != null)
	self.connect("body_entered",self,"body_entered")
	self.connect("body_exited",self,"body_exited")

	var shape: SphereShape = SphereShape.new()
	shape.radius = (points[0] - points[1]).length()

	var collision_shape = CollisionShape.new()
	collision_shape.shape = shape
	self.add_child(collision_shape)
	self.global_transform.origin = base + points[0]

	self.set_collision_layer_bit(EntityConstants.SERVER_TERRAIN_COLLISION_LAYER,false)
	self.set_collision_mask_bit(EntityConstants.SERVER_TERRAIN_COLLISION_LAYER,false)

	self.set_collision_layer_bit(EntityConstants.SERVER_PLAYER_COLLISION_LAYER,true)
	self.set_collision_mask_bit(EntityConstants.SERVER_PLAYER_COLLISION_LAYER,true)



func body_entered(body):
	assert(false)



	

