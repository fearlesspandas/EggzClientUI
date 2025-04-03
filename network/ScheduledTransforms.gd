extends Node

onready var ref = load("res://native_lib/ScheduledTransforms.gdns").new()

var initialized = false

func _ready():
	print_debug("Scheduled Transforms Ready")
	NetworkConfig.connect("physics_url_set",self,"start_socket")
	#self.add_child(ref)

func start_socket(url:String):
	if not self.initialized:
		ref.start_socket(url)
		self.initialized = true

func initialize_socket():
	if not self.initialized:
		var url = NetworkConfig.physics_host
		ref.start_socket(url)
		self.initialized = true
