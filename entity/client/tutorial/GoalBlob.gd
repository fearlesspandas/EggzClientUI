extends Area
class_name GoalBlob
signal collided(id,body)
onready var mesh_instance:MeshInstance = MeshInstance.new()
onready var collision_shape : CollisionShape = CollisionShape.new()

var id:int
var radius = 10
func _ready():
	assert(id != null)
	#initialize mesh
	var mesh : SphereMesh = SphereMesh.new()
	var material:SpatialMaterial = SpatialMaterial.new()

	mesh.radius = radius
	mesh.height = 2 * mesh.radius

	material.albedo_color = Color.orange
	material.albedo_color.a = 0.5

	mesh.material = material
	mesh_instance.mesh = mesh

	self.add_child(mesh_instance)

	#initialize collision
	var shape:SphereShape = SphereShape.new()
	shape.radius = radius
	collision_shape.shape = shape
	self.add_child(collision_shape)
	#set masks
	self.set_collision_mask_bit(EntityConstants.SERVER_TERRAIN_COLLISION_LAYER,false)
	self.set_collision_mask_bit(EntityConstants.CLIENT_PLAYER_COLLISION_LAYER,true)
	self.set_collision_layer_bit(EntityConstants.CLIENT_PLAYER_COLLISION_LAYER,true)
	self.connect("body_entered",self,"entered")


func entered(body):
	print_debug("Target Hit!")
	emit_signal("collided",id,body)

func despawn():
	
	get_parent().remove_child(self)
	self.queue_free()
