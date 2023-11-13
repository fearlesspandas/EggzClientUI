extends Spatial


# Declare member variables here. Examples:
# var a = 2
# var b = "text"
#onready var player_cam = find_node("PlayerCam")
onready var camera_root =find_node("CameraRoot")
onready var camera:Camera = camera_root.find_node("Camera")
onready var curserRay:RayCast = camera_root.find_node("CursorRay")
onready var clientPlayerEntity = find_node("ClientPlayerEntity")
onready var entity = clientPlayerEntity.body
onready var message_controller = clientPlayerEntity.message_controller
onready var id = entity.id
func _input(event):
	if event is InputEventMouseButton and event.is_action_pressed("left_click") and curserRay.intersect_position != null:
		print("attempting to add destination")
		var y = entity.body.global_transform.origin.y
		var x = curserRay.intersect_position.x
		var z = curserRay.intersect_position.z
		var dest = Vector3(x,y,z)
		if entity.id != null:
			print("entity id is not null")
			ServerNetwork.add_destination(entity.id,dest)
func _physics_process(delta):
	#aligns player space with kinematic body present clientplayerentity
	#because cameraroot is a sub node it also follows the player body automatically
	self.global_transform.origin = entity.body.global_transform.origin

func _ready():
	
	pass # Replace with function body.
func init_with_id(newid):
	entity.id = newid
	id = newid
