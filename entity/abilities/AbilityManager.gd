extends Node

var client_spawn:Spatial
var server_spawn:Spatial
var client_id_server:String
#func _ready():
#	assert(spawn != null)

func ability_client(ability_id:int,location:Vector3):
	match ability_id:
		0:
			var sc = SmackClient.new()
			client_spawn.add_child(sc)
			sc.global_transform.origin = location
		_:
			print_debug("No ability found with id ", ability_id)




func ability_server(ability_id:int,location:Vector3):
	match ability_id:
		0:
			var sc = SmackServer.new()
			server_spawn.add_child(sc)
			sc.global_transform.origin = location
		_:
			print_debug("No ability found with id ", ability_id)


func do_ability_server(ability_id:int,entity_id:String):
	match ability_id:
		0:
			ServerNetwork.get(client_id_server).remove_health(entity_id,10)
		_:
			print_debug("No ability behavior defined for id ",ability_id)
