extends Area

class_name NPCSmackServer

onready var collision_shape:CollisionShape = CollisionShape.new()
onready var despawn_timer:Timer = Timer.new()

func _ready():
	var shape:SphereShape = SphereShape.new()
	shape.radius = 10
	self.collision_shape.shape = shape
	self.add_child(collision_shape)

	self.set_collision_mask_bit(EntityConstants.SERVER_NPC_COLLISION_LAYER,false)
	self.set_collision_layer_bit(EntityConstants.SERVER_NPC_COLLISION_LAYER,false)
	self.set_collision_mask_bit(EntityConstants.SERVER_TERRAIN_COLLISION_LAYER,false)
	self.set_collision_layer_bit(EntityConstants.SERVER_TERRAIN_COLLISION_LAYER,false)
	self.set_collision_layer_bit(EntityConstants.SERVER_PLAYER_COLLISION_LAYER,true)
	self.set_collision_mask_bit(EntityConstants.SERVER_PLAYER_COLLISION_LAYER,true)
	
	self.connect("body_entered",self,"body_entered")

	#timer
	despawn_timer.wait_time = 1
	despawn_timer.connect("timeout",self,"despawn")
	self.add_child(despawn_timer)
	despawn_timer.start()



func body_entered(body):
	if body is ServerEntityKinematicBody:
		if body.has_method("handle_ability_collision"):
			body.handle_ability_collision(0)

func despawn():
	get_parent().remove_child(self)
	self.call_deferred("free")
