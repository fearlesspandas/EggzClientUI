extends Spatial

class_name AbilityOptionCreator

onready var sizing_mesh:MeshInstance = MeshInstance.new()
onready var sizing_mesh_body:SphereMesh = SphereMesh.new()
onready var sizing_mesh_material:SpatialMaterial = SpatialMaterial.new()
onready var ability_type:String
onready var color:Color

var client_id:String
var center:Vector3

func _ready():
	assert(client_id != null)
	assert(center != null)
	assert(ability_type != null)
	assert(color != null)
	print_debug("type ", ability_type , "color ", color)
	
	sizing_mesh_body.material = sizing_mesh_material
	sizing_mesh_body.radius = 5
	sizing_mesh.mesh = sizing_mesh_body
	self.add_child(sizing_mesh)
	Input.mouse_mode = Input.MOUSE_MODE_CONFINED
	sizing_mesh.global_transform.origin = center

func _process(delta):
	sizing_mesh_material.albedo_color = color
	var viewport_size = get_viewport().size
	var mouse_pos = get_viewport().get_mouse_position()
	var input_mouse_pos = Vector2(min(mouse_pos.x,viewport_size.x),min(mouse_pos.y,viewport_size.y))
	var position:Vector3 
	var res = CameraUtils.find_mouse_collision_or_null(get_viewport().get_camera(),get_world(),input_mouse_pos)
	if !res.empty():
		position = res.position
		var dist:float = (position - center).length()
		sizing_mesh_body.radius = dist
	else:
		var center_on_screen = get_viewport().get_camera().unproject_position(center)
		sizing_mesh_body.radius = (center_on_screen - input_mouse_pos).length()/2
	
	
func release():
	Input.mouse_mode = Input.MOUSE_MODE_VISIBLE
	get_parent().remove_child(self)
	self.call_deferred("free")
	
func _input(event):
	if event is InputEvent and (event.is_action_pressed("ui_cancel") or event.is_action_pressed("right_click")):
		release()
	if event is InputEventMouseButton and event.is_action_pressed("left_click"):
		var socket = ServerNetwork.get(client_id)
		assert(socket != null)
		var position:Vector3
		var res = CameraUtils.find_mouse_collision_or_null(get_viewport().get_camera(),get_world(),event.position)
		#if !res.empty():
		var dist = sizing_mesh_body.radius
		socket.add_destination(client_id,center,ability_type,dist)
		release()
