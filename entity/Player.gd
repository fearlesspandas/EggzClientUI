extends Spatial


# Declare member variables here. Examples:
# var a = 2
# var b = "text"
#onready var player_cam = find_node("PlayerCam")
onready var camera_root =find_node("CameraRoot")
onready var camera:Camera = camera_root.find_node("Camera")
onready var curserRay:RayCast = camera_root.find_node("CursorRay")
onready var clientPlayerEntity = find_node("ClientPlayerEntity")
onready var entity = clientPlayerEntity.entity.find_node("PhysicalPlayerEntity")
# Called when the node enters the scene tree for the first time.
var dest = null
func _input(event):
	if event is InputEventMouseButton and event.is_action_pressed("left_click") and curserRay.intersect_position != null:
		var y = entity.body.global_transform.origin.y
		var x = curserRay.intersect_position.x
		var z = curserRay.intersect_position.z
		dest = Vector3(x,y,z)
func _physics_process(delta):
	if dest != null:
		var dir = self.global_transform.origin - dest
		if dir.length() > 5:
		#currently have to set self position then body
		#self.global_transform.origin -= dir * delta
			entity.body.move_and_collide(-dir * delta * 0.001,false)
		else:
			entity.body
			dest = null
	#aligns player space with kinematic body present clientplayerentity
	#because cameraroot is a sub node it also follows the player body automatically
	self.global_transform.origin = entity.body.global_transform.origin

func _ready():
	
	pass # Replace with function body.
