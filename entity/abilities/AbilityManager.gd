extends Node

var spawn:Spatial

#func _ready():
#	assert(spawn != null)

func ability_client(ability_id:int,location:Vector3):
	match ability_id:
		0:
			var sc = SmackClient.new()
			spawn.add_child(sc)
			sc.global_transform.origin = location
		_:
			print_debug("No ability found with id ", ability_id)

