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
		var id = str(int(rand_range(-999,999)))
		test_vec_delete = Vector3(rand_range(-10,10),rand_range(-10,10),rand_range(-10,10))
		#print_debug("adding terrain",id,test_vec_delete)
		#ServerNetwork.get(client_id).create_terrain(id,test_vec_delete)
		ServerNetwork.get(client_id).get_all_terrain(client_id,nonrelative)


func set_active(active:bool):
	is_active = active
