extends Spatial

class_name Chunk

var client_id
var uuid
var spawn
var center:Vector3
var radius:float
var entity_manager:EntityManagement
onready var timer:Timer = Timer.new()
var has_loaded = false
func _ready():
	timer.wait_time = 2
	timer.connect("timeout",self,"_check_load")
	self.add_child(timer)
	timer.start()
	pass
func load_terrain():	
	ServerNetwork.get(client_id).get_cached_terrain(uuid)
	has_loaded = true

func _check_load():
	var entities = entity_manager.client_entities.values() + entity_manager.server_entities.values()
	#print_debug("null entities")
	if entities != null:
		#print_debug("Non null entities")
		for e in entities:
			var loc:Vector3 = e.global_transform.origin
			if !has_loaded and abs(loc.x - center.x) < radius and abs(loc.y - center.y) < radius and abs(loc.z - center.z) < radius:
				print_debug("loading terrain")
				load_terrain()
