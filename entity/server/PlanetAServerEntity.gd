extends ServerEntity

class_name PlanetAServerEntity

onready var gravity_box:GravityBox = GravityBox.new()
onready var collision_box:CollisionBox = CollisionBox.new()

onready var gravity_center = KinematicBody.new()
func _ready():
	DataCache.add_data(self.id,'speed',75.0)
	self.movement.physics_shared_native_socket = self.physics_native_shared_socket
	ScheduledTransforms.ref.set_mass(self.id,3000000.0)
	self.body.add_child(gravity_box)
	var size = 512.0 * 64.0
	gravity_box.ref.set_unaffected_radius(1024 * 1.2)
	gravity_box.ref.set_affected_radius(size)
	gravity_box.ref.set_id(self.id)

	gravity_box.ref.connect("entered_affected",self,"add_affected")
	gravity_box.ref.connect("exited_affected",self,"remove_affected")
	gravity_box.ref.connect("entered_unaffected",self,"add_unaffected")
	gravity_box.ref.connect("exited_unaffected",self,"remove_unaffected")

	#copied from npc server entity
	self.is_npc = true

	self.body.set_collision_layer_bit(EntityConstants.SERVER_GRAVITY_COLLISION_LAYER,true)

	var gravity_collider = CollisionShape.new()
	var collider_shape = SphereShape.new()
	gravity_collider.shape = collider_shape
	gravity_center.add_child(gravity_collider)
	gravity_center.set_collision_layer_bit(EntityConstants.SERVER_GRAVITY_COLLISION_LAYER,true)

	self.body.add_child(collision_box)
	collision_box.ref.set_radius(1024.0)

	self.add_child(gravity_center)

func add_affected(entity_id):
	pass
func remove_affected(entity_id):
	pass
func add_unaffected(entity_id):
	pass
	#self.body.get_node("CollisionShape").disabled = false
func remove_unaffected(entity_id):
	pass
	#self.body.get_node("CollisionShape").disabled = true

func _physics_process(delta):
	default_physics_process(delta)
	gravity_center.global_transform.origin = self.body.global_transform.origin
	#self.body.global_transform.origin.x += delta * 10

func init_with_id(id,client_id):
	id.erase(16,64)
	self.id = id
	self.client_id = client_id

