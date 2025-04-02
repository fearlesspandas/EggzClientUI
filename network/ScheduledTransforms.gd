extends Node

onready var ref = load("res://native_lib/ScheduledTransforms.gdns").new()

var initialized = false

func _ready():
	print_debug("Scheduled Transforms Ready")
	#self.add_child(ref)

func initialize_socket():
	if not self.initialized:
		var url = NetworkConfig.physics_host
		ref.start_socket(url)
		self.initialized = true
