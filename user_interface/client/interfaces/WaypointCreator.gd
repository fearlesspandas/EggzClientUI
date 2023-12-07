extends Spatial

class_name WaypointCreator

onready var sizing_mesh:MeshInstance = MeshInstance.new()
onready var sizing_mesh_body:CylinderMesh = CylinderMesh.new()
onready var sizing_mesh_material:SpatialMaterial = SpatialMaterial.new()

var client_id:String
var center:Vector3

func _ready():
	assert(client_id != null)
	assert(center != null)
	sizing_mesh_material.albedo_color = Color.purple
	sizing_mesh_body.material = sizing_mesh_material
	sizing_mesh_body.height = 5
	sizing_mesh.mesh = sizing_mesh_body
	self.add_child(sizing_mesh)
	Input.mouse_mode = Input.MOUSE_MODE_CONFINED
	sizing_mesh.global_transform.origin = center

func _process(delta):
	var position:Vector3 = CameraUtils.find_mouse_collision(get_viewport().get_camera(),get_world(),get_viewport().get_mouse_position())
	var dist:float = (position - center).length()
	sizing_mesh_body.top_radius = dist
	sizing_mesh_body.bottom_radius = dist
	
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
		var position:Vector3 = CameraUtils.find_mouse_collision(get_viewport().get_camera(),get_world(),event.position)
		var dist = (position - center).length() #send distance as radius
		socket.add_destination(client_id,center,"WAYPOINT",dist)
		release()
