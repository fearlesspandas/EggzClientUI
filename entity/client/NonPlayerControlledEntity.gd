extends ClientPlayerEntity

func _physics_process(delta):
	#aligns player space with kinematic body present clientplayerentity
	#because cameraroot is a sub node it also follows the player body automatically
	#self.global_transform.origin = entity.body.global_transform.origin
	pass
	

func _ready():
	pass # Replace with function body.
