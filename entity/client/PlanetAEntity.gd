extends ClientPlayerEntity
class_name PlanetAEntity
onready var collision_box:CollisionBox = CollisionBox.new()

func _ready():
	collision_box.ref.set_server(false)
	self.body.add_child(collision_box)
	collision_box.ref.set_radius(2048.0)
	collision_box.ref.set_movement_body(self.body)
	collision_box.ref.set_collision_layer_bit(EntityConstants.CLIENT_MOUSE_RAYCAST_COLLISION_LAYER,true)
	collision_box.ref.connect("body_clicked",self,"add_entity_data_to_terminal")

func add_entity_data_to_terminal():
	ClientTerminalGlobalSignals.add_input_data("Id ",self.id)
	ClientTerminalGlobalSignals.add_input_data("dir (velocity)" ,str(movement.dir.length()))
	ClientTerminalGlobalSignals.add_input_data("global_position" , str(body.global_transform.origin))
	ClientTerminalGlobalSignals.add_input_data("bytes_received_mb" ,str(float(physics_native_shared_socket.num_bytes_received(id))/1000000.0))
	
func _physics_process(delta):
	default_physics_process(delta,2)

func _handle_message(msg,delta_accum):
	self.default_handle_message(msg,delta_accum)

func init_with_id(id,client_id):
	id.erase(16,64)
	self.id = id
	self.client_id = client_id

