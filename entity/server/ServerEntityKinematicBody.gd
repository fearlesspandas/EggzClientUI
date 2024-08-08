extends KinematicBody

# Called when the node enters the scene tree for the first time.
onready var parent:ServerEntity = get_parent()
func _ready():
	if !parent.is_npc:
		self.set_collision_layer_bit(10,true)
		self.set_collision_layer_bit(0,true)
		self.set_collision_mask_bit(10,true)
		self.set_collision_mask_bit(0,true)


func not_process(delta):
	if Input.is_action_just_pressed("fall"):
		var teleport_distance = 1
		var forward_direction = - global_transform.basis.z

		var teleport_vector = forward_direction.normalized() * -teleport_distance

		var teleport_position = translation + teleport_vector

		var collision = move_and_collide(teleport_vector)

		if collision:

			teleport_position = collision.position

		translation = teleport_position
