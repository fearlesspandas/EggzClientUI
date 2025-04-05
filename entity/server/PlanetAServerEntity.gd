extends ServerEntity

class_name PlanetAServerEntity

onready var gravity_box:GravityBox = GravityBox.new()

func _ready():
	self.movement.physics_shared_native_socket = self.physics_native_shared_socket
	ScheduledTransforms.ref.set_mass(self.id,3000000.0)
	self.add_child(gravity_box)
	var size = 512.0 * 64.0
	gravity_box.ref.set_unaffected_radius(1024 * 1.2)
	gravity_box.ref.set_affected_radius(size)
	gravity_box.ref.set_id(self.id)

	#copied from npc server entity
	self.is_npc = true

func _physics_process(delta):
	default_physics_process(delta)
