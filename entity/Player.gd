extends ClientPlayerEntity


onready var camera_root =find_node("CameraRoot")
onready var camera:Camera = camera_root.find_node("Camera")
onready var curserRay:RayCast = camera_root.find_node("CursorRay")
func _input(event):
	if event is InputEventMouseButton and event.is_action_pressed("left_click") and curserRay.intersect_position != null:
		#print("attempting to add destination")
		var y = body.global_transform.origin.y
		var x = curserRay.intersect_position.x
		var z = curserRay.intersect_position.z
		var dest = Vector3(x,y,z)
		if id != null:
			#print("entity id is not null")
			ServerNetwork.socket.add_destination(id,dest)
func _physics_process(delta):
	#aligns player space with kinematic body present clientplayerentity
	#because cameraroot is a sub node it also follows the player body automatically
	#self.global_transform.origin = entity.body.global_transform.origin
	pass
func _ready():
	pass # Replace with function body.
