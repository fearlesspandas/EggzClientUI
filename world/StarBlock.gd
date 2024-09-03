extends StaticBody
class_name StarBlock
onready var health_replenish_timer:Timer = Timer.new()
onready var collision_shape:CollisionShape = CollisionShape.new()
onready var box_shape:BoxShape = BoxShape.new()

var MAX_HEALTH = 10
var available_health = MAX_HEALTH
var uuid:String
var spawn_mesh=true
func _ready():
	box_shape.extents = Vector3(1,1,1)
	collision_shape.shape = box_shape
	self.add_child(collision_shape)
	if spawn_mesh:
		spawn_mesh_instance()
	health_replenish_timer.wait_time = 1
	health_replenish_timer.connect("timeout",self,"replenish_health")
	self.add_child(health_replenish_timer)
	
	self.set_collision_layer_bit(12,true)
	self.set_collision_mask_bit(12,true)

func spawn_mesh_instance():
	var mesh_instance = MeshInstance.new()
	var mesh:CubeMesh = CubeMesh.new()
	mesh.size = 2*box_shape.extents
	mesh.material = SpatialMaterial.new()
	mesh.material.albedo_color = Color(0,1,0.917647,1)
	mesh_instance.mesh = mesh
	self.add_child(mesh_instance)
	
func handle_collision(client_id:String,player_id:String):
	if available_health > 0:
		ServerNetwork.get(client_id).add_health(player_id,available_health)
		var stats = {'max_speed_delta':available_health,'speed_delta':0}
		ServerNetwork.get(client_id).adjust_max_speed(player_id,available_health)
		available_health = 0
	health_replenish_timer.start()

func replenish_health():
	if available_health < MAX_HEALTH:
		available_health += 1
	else:
		health_replenish_timer.stop()

func init_with_id(uuid:String):
	self.uuid = uuid
