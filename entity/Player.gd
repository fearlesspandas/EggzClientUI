extends ClientPlayerEntity


onready var camera_root =find_node("CameraRoot")
onready var camera:Camera = camera_root.find_node("Camera")
onready var curserRay:RayCast = camera_root.find_node("CursorRay")
var is_active = true
func _input(event):
	if is_active and event is InputEventMouseButton and event.is_action_pressed("left_click") and curserRay.intersect_position != null:
		#print("attempting to add destination")
		var y = body.global_transform.origin.y
		var x = curserRay.intersect_position.x
		var z = curserRay.intersect_position.z
		var dest = Vector3(x,y,z)
		if id != null:
			print("setting destination for player:",dest)
			ServerNetwork.get(id).add_destination(id,dest)
func _physics_process(delta):
	#aligns player space with kinematic body present clientplayerentity
	#because cameraroot is a sub node it also follows the player body automatically
	#self.global_transform.origin = entity.body.global_transform.origin
	pass
	
func set_active(active:bool):
	print("player active:",active)
	is_active = active
func _ready():
	pass # Replace with function body.
