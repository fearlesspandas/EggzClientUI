extends Timer

class_name TerrainScannerTimer

var client_id
var is_active:bool = false
var nonrelative:bool
var test_vec_delete = Vector3(0,0,0)

func _ready():
	self.connect("timeout",self,"poll_for_terrain")
	
	pass

func poll_for_terrain():
	if is_active:
		#ServerNetwork.get(client_id).get_top_level_terrain()#.get_all_terrain(client_id,true)
		ServerNetwork.get(client_id).get_top_level_terrain_in_distance(500,Vector3(0,0,0))
		one_shot = true

func set_active(active:bool):
	is_active = active
