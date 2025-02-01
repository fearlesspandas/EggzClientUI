extends Area
class_name GoalBlob
signal collided(id,entity_id)
onready var mesh_instance:MeshInstance = MeshInstance.new()
onready var collision_shape : CollisionShape = CollisionShape.new()
onready var color_timer:Timer = Timer.new()

var id:int
var radius = 10
var colors = [Color.aqua,Color.blue,Color.blueviolet,Color.aquamarine,]
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
	self.set_collision_layer_bit(EntityConstants.SERVER_TERRAIN_COLLISION_LAYER,false)
	self.set_collision_mask_bit(EntityConstants.SERVER_NPC_COLLISION_LAYER,false)
	self.set_collision_layer_bit(EntityConstants.SERVER_NPC_COLLISION_LAYER,false)
	self.set_collision_mask_bit(EntityConstants.SERVER_PLAYER_COLLISION_LAYER,false)
	self.set_collision_layer_bit(EntityConstants.SERVER_PLAYER_COLLISION_LAYER,false)
	self.set_collision_mask_bit(EntityConstants.CLIENT_PLAYER_COLLISION_LAYER,true)
	self.set_collision_layer_bit(EntityConstants.CLIENT_PLAYER_COLLISION_LAYER,false)
	self.connect("body_entered",self,"entered")

	#timer
	color_timer.wait_time = 1
	color_timer.connect("timeout",self,"random_color")
	self.add_child(color_timer)
	color_timer.start()
	random_color()


func entered(body):
	if body is ClientEntityKinematicBody:
		emit_signal("collided",id,body.client_player_entity.id)

func despawn():
	get_parent().remove_child(self)
	self.call_deferred("free")

var proc_1:int = 0
var proc_2:int = 0
func color_processes(delta):
	if proc_1%256 == 0:
		proc_1 = 0
		proc_2 += 1
		mesh_instance.mesh.material.albedo_color = Color.white
			
	else:		
		if proc_2%3 == 0:
			proc_2 = 0
			mesh_instance.mesh.material.albedo_color.r += 1

		if proc_2%3 == 1:
			mesh_instance.mesh.material.albedo_color.g += 1

		if proc_2%3 == 2:
			mesh_instance.mesh.material.albedo_color.b += 1
	proc_1 += 1			

func set_color(color:Color):
	mesh_instance.mesh.material.albedo_color = color
	
func random_color():
	var rand_x = int(rand_range(0,255))
	var rand_y = int(rand_range(0,255))
	var rand_z = int(rand_range(0,255))
	var rand_ind = int(rand_range(0,colors.size()))
	mesh_instance.mesh.material.albedo_color = colors[rand_ind]

func stop():
	color_timer.stop()

