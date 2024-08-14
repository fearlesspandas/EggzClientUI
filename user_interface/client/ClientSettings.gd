extends Node


var CHUNK_REQUEST_SIZE = 1024
var CHUNK_DISTANCE_ON_PLAYER_LOAD = 1024
signal chunk_request_radius_multiplier(value)
var CHUNK_REQUEST_RADIUS_MULTIPLIER = 4
var LOAD_RECEIVED_CHUNK_IF_WITHIN = 1024
signal camera_render_distance(value)
var CAMERA_RENDER_DISTANCE = 1024

func set_render_distance(value:float):
	CAMERA_RENDER_DISTANCE = value
	emit_signal("camera_render_distance",value)
	
func set_chunk_request_radius_multiplier(value:int):
	CHUNK_REQUEST_RADIUS_MULTIPLIER = value
	emit_signal("chunk_request_radius_multiplier",value)
	
func link_handler(handler:Node):
	assert(handler.has_method("camera_render_distance"))
	self.connect("camera_render_distance",handler,"camera_render_distance")
	assert(handler.has_method("chunk_request_radius_multiplier"))
	self.connect("chunk_request_radius_multiplier",handler,"chunk_request_radius_multiplier")
	
func camera_render_distance(value:float):
	var vp = ClientReferences.viewport
	if vp != null:
		var cam = vp.get_camera()
		if cam != null:
			cam.far = value
			print("render distance set " , value)

func chunk_request_radius_multiplier(value:int):
	pass

func _ready():
	link_handler(self)
	
func _process(delta):
	if Input.is_action_pressed("increase_render_distance"):
		set_render_distance(CAMERA_RENDER_DISTANCE + 2)
	if Input.is_action_pressed("decrease_render_distance"):
		set_render_distance(CAMERA_RENDER_DISTANCE - 2)
